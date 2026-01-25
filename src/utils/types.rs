use serde::{Deserialize, Serialize};

/// Represents an individual project item displayed on the projects page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectItem {
    pub name: String,
    pub desc: String,
    pub tech: Vec<String>,
    pub link: String,
    pub status: String,
}

/// Configuration for a post (metadata only, no content).
/// Used in posts.json listing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostConfig {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
    #[serde(default)]
    pub projects: Vec<ProjectItem>,
}

/// A full post with content (used for individual post pages).
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
