use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};

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

static ALL_POSTS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/posts.json"));

pub fn get_all_posts() -> Vec<PostConfig> {
    serde_json::from_str(ALL_POSTS_JSON).expect("Failed to parse embedded posts.json")
}

pub async fn fetch_post(slug: &str) -> Option<Post> {
    // Keep this function as it fetches individual markdown files
    // and parsing them at runtime for their content and frontmatter
    use gloo_net::http::Request;
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