use gray_matter::{Matter, engine::YAML};
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use base64::prelude::*;

static POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/posts");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostConfig {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
}

pub fn get_all_posts() -> Vec<Post> {
    let mut posts = Vec::new();
    let matter = Matter::<YAML>::new();

    for file in POSTS_DIR.files() {
        if let Some(content_str) = file.contents_utf8() {
            // parse returns a result into gray matter
            if let Ok(parsed) = matter.parse::<gray_matter::Pod>(content_str) {
                // Attempt to deserialize the frontmatter into PostConfig
                if let Some(data) = parsed.data {
                    let config = data.deserialize::<PostConfig>().ok();

                    if let Some(cfg) = config {
                        posts.push(Post {
                            title: cfg.title,
                            date: cfg.date,
                            slug: cfg.slug,
                            summary: cfg.summary,
                            content: parsed.content,
                        });
                    }
                }
            }
        }
    }

    // Sort by date (descending) handling DD-MM-YYYY format
    posts.sort_by(|a, b| {
        let to_sortable = |date: &str| -> String {
            let parts: Vec<&str> = date.split('-').collect();
            if parts.len() == 3 {
                // Convert DD-MM-YYYY to YYYY-MM-DD
                format!("{}-{}-{}", parts[2], parts[1], parts[0])
            } else {
                date.to_string()
            }
        };

        to_sortable(&b.date).cmp(&to_sortable(&a.date))
    });

    posts
}

pub fn get_post_by_slug(slug: &str) -> Option<Post> {
    get_all_posts().into_iter().find(|p| p.slug == slug)
}

pub fn get_post_image(path: &str) -> Option<String> {
    let clean_path = path.trim_start_matches("./").trim_start_matches("/");
    
    let file = POSTS_DIR.get_file(clean_path)?;
    let content = file.contents();
    
    let extension = std::path::Path::new(clean_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("octet-stream");
        
    let mime_type = match extension.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    };
    
    let b64 = BASE64_STANDARD.encode(content);
    Some(format!("data:{};base64,{}", mime_type, b64))
}
