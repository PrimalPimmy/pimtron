use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectItem {
    pub name: String,
    pub desc: String,
    pub tech: Vec<String>,
    pub link: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostConfig {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
    #[serde(default)]
    pub projects: Vec<ProjectItem>,
}

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

pub async fn fetch_all_posts() -> Vec<PostConfig> {
    use gloo_net::http::Request;
    // Standard GET request, browser will handle caching automatically
    let response = Request::get("/posts/posts.json").send().await;
    
    if let Ok(resp) = response
        && resp.ok() {
            return resp.json::<Vec<PostConfig>>().await.unwrap_or_default();
        }
    Vec::new()
}

pub async fn fetch_post(slug: &str) -> Option<Post> {
    use gloo_net::http::Request;
    let url = format!("/posts/{}.json", slug);
    let response = Request::get(&url).send().await.ok()?;
    
    if !response.ok() {
        return None;
    }

    response.json::<Post>().await.ok()
}

pub async fn fetch_about() -> Option<Post> {
    use gloo_net::http::Request;
    let url = "/posts/about.json";
    let response = Request::get(url).send().await.ok()?;

    if !response.ok() {
        return None;
    }

    response.json::<Post>().await.ok()
}

pub async fn fetch_projects() -> Option<Post> {
    use gloo_net::http::Request;
    let url = "/posts/projects.json";
    let response = Request::get(url).send().await.ok()?;

    if !response.ok() {
        return None;
    }

    response.json::<Post>().await.ok()
}
