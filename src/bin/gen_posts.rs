use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use syntect::{
    highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet,
};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostConfig {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
}

fn highlight_code(events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    let mut in_code_block = false;

    let syntax_set = SyntaxSet::load_defaults_nonewlines();
    let mut syntax = syntax_set.find_syntax_plain_text();

    // Load theme from file
    let theme_str = include_str!("../components/rose-pine.tmTheme");
    let theme = ThemeSet::load_from_reader(&mut Cursor::new(theme_str)).unwrap();

    let mut to_highlight = String::new();
    let mut out_events = Vec::new();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                match kind {
                    CodeBlockKind::Fenced(lang) => {
                        syntax = syntax_set.find_syntax_by_token(&lang).unwrap_or(syntax)
                    }
                    CodeBlockKind::Indented => {}
                }
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                if !in_code_block {
                    panic!("this should never happen");
                }
                let html = highlighted_html_for_string(
                    &to_highlight,
                    &syntax_set,
                    syntax,
                    &theme,
                )
                .unwrap();

                to_highlight.clear();
                in_code_block = false;
                out_events.push(Event::Html(CowStr::from(html)));
            }
            Event::Text(t) => {
                if in_code_block {
                    to_highlight.push_str(&t);
                } else {
                    out_events.push(Event::Text(t));
                }
            }
            e => {
                out_events.push(e);
            }
        }
    }

    out_events
}

fn main() {
    println!("Generating posts...");

    let generated_posts_dir = Path::new("generated_posts");

    if !generated_posts_dir.exists() {
        fs::create_dir(generated_posts_dir).expect("Failed to create 'generated_posts' directory");
    }

    let mut posts = Vec::new();
    // Temporary storage to hold parsed content before final formatting
    let mut raw_posts_data: Vec<(PostConfig, String)> = Vec::new();

    for entry in fs::read_dir("posts").expect("Failed to read 'posts' directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            println!("Processing {:?}", path);
            let content_str = fs::read_to_string(&path).expect("Failed to read post file content");
            let matter = Matter::<YAML>::new();
            
            if let Ok(parsed) = matter.parse::<PostConfig>(&content_str) {
                if let Some(config) = parsed.data {
                    // Store the config and the raw markdown content
                    raw_posts_data.push((config, parsed.content));
                }
            } else {
                 eprintln!("Failed to parse frontmatter for {:?}", path);
            }
        }
    }

    // Sort by date (ISO 8601 strings sort correctly)
    raw_posts_data.sort_by(|a, b| b.0.date.cmp(&a.0.date));

    for (mut config, raw_content) in raw_posts_data {
        // Parse date and reformat for display
        if let Ok(date) = NaiveDate::parse_from_str(&config.date, "%Y-%m-%d") {
            config.date = date.format("%B %d, %Y").to_string();
        } else {
            eprintln!("Warning: Could not parse date '{}' for post '{}'. Keeping original.", config.date, config.slug);
        }

        // Generate HTML content
        let parser = Parser::new(&raw_content);
        let events: Vec<_> = parser.collect();
        let highlighted_events = highlight_code(events);

        let mut html_output = String::new();
        html::push_html(&mut html_output, highlighted_events.into_iter());

        let post = Post {
            title: config.title.clone(),
            date: config.date.clone(), // Use formatted date
            slug: config.slug.clone(),
            summary: config.summary.clone(),
            content: html_output,
        };

        // Write individual post JSON
        let json_filename = format!("{}.json", config.slug);
        let json_path = generated_posts_dir.join(json_filename);
        let json_str = serde_json::to_string(&post).unwrap();
        fs::write(json_path, json_str).unwrap();

        // Add to the list for posts.json
        posts.push(config);
    }

    // Write the sorted and formatted list of posts to generated_posts/posts.json
    let dest_path = generated_posts_dir.join("posts.json");
    let json = serde_json::to_string(&posts).unwrap();
    fs::write(dest_path, json).unwrap();
    
    println!("Done!");
}
