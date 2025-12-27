use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use web_sys::console;

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

pub async fn fetch_all_posts() -> Vec<PostConfig> {
    match Request::get("/posts/index.json").send().await {
        Ok(resp) => {
            if !resp.ok() {
                console::log_1(&format!("Failed to fetch posts: Status {}", resp.status()).into());
                return Vec::new();
            }
            match resp.json().await {
                Ok(posts) => posts,
                Err(e) => {
                    console::log_1(&format!("Failed to parse posts JSON: {}", e).into());
                    Vec::new()
                }
            }
        },
        Err(e) => {
            console::log_1(&format!("Network error fetching posts: {}", e).into());
            Vec::new()
        }
    }
}

pub async fn fetch_post(slug: &str) -> Option<Post> {
    let url = format!("/posts/{}.md", slug);
    let response = Request::get(&url).send().await.ok()?;
    
    if !response.ok() {
        return None;
    }

    let content_str = response.text().await.ok()?;
    let matter = Matter::<YAML>::new();

    if let Ok(parsed) = matter.parse::<gray_matter::Pod>(&content_str)
        && let Some(data) = parsed.data {
            let config = data.deserialize::<PostConfig>().ok();

            if let Some(cfg) = config {
                return Some(Post {
                    title: cfg.title,
                    date: cfg.date,
                    slug: cfg.slug,
                    summary: cfg.summary,
                    content: parsed.content,
                });
            }
        }
    None
}