use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use syntect::{
    highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet,
};

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
    pub content: String, // This will now contain HTML
}

fn highlight_code(events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    let mut in_code_block = false;

    let syntax_set = SyntaxSet::load_defaults_nonewlines();
    let mut syntax = syntax_set.find_syntax_plain_text();

    // Load theme from file relative to build script
    let theme_str = include_str!("src/components/rose-pine.tmTheme");
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
    println!("cargo:rerun-if-changed=posts");
    println!("cargo:rerun-if-changed=src/components/rose-pine.tmTheme");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("posts.json");
    let generated_posts_dir = Path::new("generated_posts");

    if !generated_posts_dir.exists() {
        fs::create_dir(generated_posts_dir).unwrap();
    }

    let mut posts = Vec::new();

    for entry in fs::read_dir("posts").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            let content_str = fs::read_to_string(&path).unwrap();
            let matter = Matter::<YAML>::new();
            
            // Parse returns a Result in this version, so we must unwrap or handle it
            if let Ok(parsed) = matter.parse::<PostConfig>(&content_str) {
                if let Some(config) = parsed.data {
                    // Add to list
                    posts.push(config.clone());

                    // Process Content -> HTML
                    let parser = Parser::new(&parsed.content);
                    let events: Vec<_> = parser.collect();
                    let highlighted_events = highlight_code(events);

                    let mut html_output = String::new();
                    html::push_html(&mut html_output, highlighted_events.into_iter());

                    // Create Post object
                    let post = Post {
                        title: config.title,
                        date: config.date,
                        slug: config.slug.clone(),
                        summary: config.summary,
                        content: html_output,
                    };

                    // Write individual JSON file to generated_posts/
                    let json_filename = format!("{}.json", config.slug);
                    let json_path = generated_posts_dir.join(json_filename);
                    let json_str = serde_json::to_string(&post).unwrap();
                    fs::write(json_path, json_str).unwrap();
                }
            } else {
                 eprintln!("Failed to parse frontmatter for {:?}", path);
            }
        }
    }

    // Sort posts by date (descending)
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    let json = serde_json::to_string(&posts).unwrap();
    fs::write(dest_path, json).unwrap();
}