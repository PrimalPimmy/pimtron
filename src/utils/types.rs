use serde::Deserialize;

/// Represents an individual project item displayed on the projects page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectItem {
    pub name: String,
    pub desc: String,
    pub tech: Vec<String>,
    pub link: String,
    pub status: String,
}

/// Configuration for a post (metadata only, no content).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostConfig {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
    pub projects: Vec<ProjectItem>,
}

/// A full post with content (used for individual post pages).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub projects: Vec<ProjectItem>,
}

// --- AT Protocol Response Structs ---

#[derive(Debug, Clone, Deserialize)]
pub struct ListRecordsResponse {
    pub records: Vec<AtprotoRecord>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtprotoRecord {
    pub uri: String,
    pub value: StandardDocument,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StandardDocument {
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub path: String,
    #[serde(rename = "publishedAt", default)]
    pub published_at: String,
    #[serde(rename = "textContent", default)]
    pub text_content: String,
    #[serde(default)]
    pub content: Option<serde_json::Value>,
}
