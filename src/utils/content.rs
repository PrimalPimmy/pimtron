use serde::{Deserialize, Serialize};

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

static ALL_POSTS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/posts.json"));

pub fn get_all_posts() -> Vec<PostConfig> {
    serde_json::from_str(ALL_POSTS_JSON).expect("Failed to parse embedded posts.json")
}

pub async fn fetch_post(slug: &str) -> Option<Post> {
    use gloo_net::http::Request;
    // Now fetching JSON which contains the pre-rendered HTML
    let url = format!("/posts/{}.json", slug);
    let response = Request::get(&url).send().await.ok()?;
    
    if !response.ok() {
        return None;
    }

    response.json::<Post>().await.ok()
}