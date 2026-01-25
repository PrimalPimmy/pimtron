//! Build script to generate JSON files from markdown posts.

use chrono::NaiveDate;
use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag, TagEnd, html};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::Path;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

/// Represents an individual project item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectItem {
    pub name: String,
    pub desc: String,
    pub tech: Vec<String>,
    pub link: String,
    pub status: String,
}

/// Configuration for a post (metadata only).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostConfig {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
    #[serde(default)]
    pub projects: Vec<ProjectItem>,
}

/// A full post with rendered HTML content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    #[serde(default)]
    pub projects: Vec<ProjectItem>,
}

/// Error type for post generation.
#[derive(Debug)]
enum GenError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Theme(String),
}

impl From<std::io::Error> for GenError {
    fn from(e: std::io::Error) -> Self {
        GenError::Io(e)
    }
}

impl From<serde_json::Error> for GenError {
    fn from(e: serde_json::Error) -> Self {
        GenError::Json(e)
    }
}

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenError::Io(e) => write!(f, "IO error: {}", e),
            GenError::Json(e) => write!(f, "JSON error: {}", e),
            GenError::Theme(e) => write!(f, "Theme error: {}", e),
        }
    }
}

/// Applies syntax highlighting to code blocks in the event stream.
fn highlight_code<'a>(
    events: Vec<Event<'a>>,
    syntax_set: &SyntaxSet,
    theme: &syntect::highlighting::Theme,
) -> Vec<Event<'a>> {
    let mut in_code_block = false;
    let mut syntax = syntax_set.find_syntax_plain_text();
    let mut to_highlight = String::new();
    let mut out_events = Vec::new();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                if let CodeBlockKind::Fenced(lang) = kind {
                    syntax = syntax_set.find_syntax_by_token(&lang).unwrap_or(syntax);
                }
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                if !in_code_block {
                    eprintln!("Warning: unexpected end of code block");
                    continue;
                }
                match highlighted_html_for_string(&to_highlight, syntax_set, syntax, theme) {
                    Ok(html) => out_events.push(Event::Html(CowStr::from(html))),
                    Err(e) => {
                        eprintln!("Warning: syntax highlighting failed: {}", e);
                        // Fallback to plain code
                        out_events.push(Event::Html(CowStr::from(format!(
                            "<pre><code>{}</code></pre>",
                            to_highlight
                        ))));
                    }
                }
                to_highlight.clear();
                in_code_block = false;
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

/// Converts markdown content to HTML with syntax highlighting.
fn markdown_to_html(
    content: &str,
    syntax_set: &SyntaxSet,
    theme: &syntect::highlighting::Theme,
) -> String {
    let parser = Parser::new(content);
    let events: Vec<_> = parser.collect();
    let highlighted_events = highlight_code(events, syntax_set, theme);
    let mut html_output = String::new();
    html::push_html(&mut html_output, highlighted_events.into_iter());
    html_output
}

/// Writes a post to a JSON file.
fn write_post_json(path: &Path, post: &Post) -> Result<(), GenError> {
    let json_str = serde_json::to_string(post)?;
    fs::write(path, json_str)?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), GenError> {
    println!("Generating posts...");

    let generated_posts_dir = Path::new("generated_posts");
    if !generated_posts_dir.exists() {
        fs::create_dir(generated_posts_dir)?;
    }

    // Load syntax highlighting resources once
    let syntax_set = SyntaxSet::load_defaults_nonewlines();
    let theme_str = include_str!("../components/rose-pine.tmTheme");
    let theme = ThemeSet::load_from_reader(&mut Cursor::new(theme_str))
        .map_err(|e| GenError::Theme(format!("{:?}", e)))?;

    let mut posts = Vec::new();
    let mut raw_posts_data: Vec<(PostConfig, String)> = Vec::new();

    for entry in fs::read_dir("posts")? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }

        println!("Processing {:?}", path);
        let content_str = fs::read_to_string(&path)?;
        let matter = Matter::<YAML>::new();

        let parsed = match matter.parse::<PostConfig>(&content_str) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to parse frontmatter for {:?}: {:?}",
                    path, e
                );
                continue;
            }
        };

        let Some(config) = parsed.data else {
            eprintln!("Warning: No frontmatter data in {:?}", path);
            continue;
        };

        // Special handling for AboutMe.md
        if filename == "AboutMe.md" {
            let html_output = markdown_to_html(&parsed.content, &syntax_set, &theme);
            let about_post = Post {
                title: config.title,
                date: config.date,
                slug: config.slug,
                summary: config.summary,
                content: html_output,
                projects: vec![],
            };

            let json_path = generated_posts_dir.join("about.json");
            write_post_json(&json_path, &about_post)?;
            println!("Generated about.json");
            continue;
        }

        // Special handling for Projects.md
        if filename == "Projects.md" {
            let html_output = markdown_to_html(&parsed.content, &syntax_set, &theme);
            let project_post = Post {
                title: config.title,
                date: config.date,
                slug: config.slug,
                summary: config.summary,
                content: html_output,
                projects: config.projects,
            };

            let json_path = generated_posts_dir.join("projects.json");
            write_post_json(&json_path, &project_post)?;
            println!("Generated projects.json");
            continue;
        }

        // Store regular posts for later processing
        raw_posts_data.push((config, parsed.content));
    }

    // Sort by date (ISO 8601 strings sort correctly)
    raw_posts_data.sort_by(|a, b| b.0.date.cmp(&a.0.date));

    for (mut config, raw_content) in raw_posts_data {
        // Format date for display
        if let Ok(date) = NaiveDate::parse_from_str(&config.date, "%Y-%m-%d") {
            config.date = date.format("%B %d, %Y").to_string();
        } else {
            eprintln!(
                "Warning: Could not parse date '{}' for post '{}'. Keeping original.",
                config.date, config.slug
            );
        }

        let html_output = markdown_to_html(&raw_content, &syntax_set, &theme);

        let post = Post {
            title: config.title.clone(),
            date: config.date.clone(),
            slug: config.slug.clone(),
            summary: config.summary.clone(),
            content: html_output,
            projects: vec![],
        };

        let json_filename = format!("{}.json", config.slug);
        let json_path = generated_posts_dir.join(&json_filename);
        write_post_json(&json_path, &post)?;

        posts.push(config);
    }

    // Write the sorted list of posts
    let dest_path = generated_posts_dir.join("posts.json");
    let json = serde_json::to_string(&posts)?;
    fs::write(dest_path, json)?;

    println!("Done!");
    Ok(())
}
