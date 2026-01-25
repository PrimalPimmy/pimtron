// Re-export types for convenience
pub use crate::utils::types::{Post, PostConfig};

/// Fetches all post configs from the posts.json listing.
pub async fn fetch_all_posts() -> Vec<PostConfig> {
    use gloo_net::http::Request;

    let response = Request::get("/posts/posts.json").send().await;

    if let Ok(resp) = response
        && resp.ok()
    {
        return resp.json::<Vec<PostConfig>>().await.unwrap_or_default();
    }
    Vec::new()
}

/// Fetches a single post by slug.
pub async fn fetch_post(slug: &str) -> Option<Post> {
    use gloo_net::http::Request;

    let url = format!("/posts/{}.json", slug);
    let response = Request::get(&url).send().await.ok()?;

    if !response.ok() {
        return None;
    }

    response.json::<Post>().await.ok()
}

/// Fetches the about page content.
pub async fn fetch_about() -> Option<Post> {
    use gloo_net::http::Request;

    let response = Request::get("/posts/about.json").send().await.ok()?;

    if !response.ok() {
        return None;
    }

    response.json::<Post>().await.ok()
}

/// Fetches the projects page content.
pub async fn fetch_projects() -> Option<Post> {
    use gloo_net::http::Request;

    let response = Request::get("/posts/projects.json").send().await.ok()?;

    if !response.ok() {
        return None;
    }

    response.json::<Post>().await.ok()
}
