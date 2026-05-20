pub use crate::utils::types::{Post, PostConfig};

const ATPROTO_DID: &str = "did:plc:kdcc475mwmfd7ehlvhqxgrqr";
const COLLECTION: &str = "site.standard.document";

pub async fn fetch_all_posts() -> Vec<PostConfig> {
    fetch_all_posts_inner().await.unwrap_or_default()
}

async fn fetch_all_posts_inner() -> Option<Vec<PostConfig>> {
    use crate::utils::types::ListRecordsResponse;
    use gloo_net::http::Request;

    let url = format!(
        "https://bsky.social/xrpc/com.atproto.repo.listRecords?repo={}&collection={}",
        ATPROTO_DID, COLLECTION
    );
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let data = resp.json::<ListRecordsResponse>().await.ok()?;

    let mut posts: Vec<PostConfig> = data
        .records
        .into_iter()
        .filter_map(|record| {
            let rkey = record.uri.rsplit('/').next().unwrap_or_default();
            let slug = record.value.path.rsplit('/').next().unwrap_or_default();
            let slug = if slug.is_empty() {
                rkey.to_owned()
            } else {
                slug.to_owned()
            };

            // Skip special pages
            if matches!(rkey, "aboutme" | "projects")
                || matches!(slug.to_lowercase().as_str(), "aboutme" | "projects")
            {
                return None;
            }

            let date = chrono::DateTime::parse_from_rfc3339(&record.value.published_at)
                .map(|dt| dt.format("%B %d, %Y").to_string())
                .unwrap_or(record.value.published_at);

            Some(PostConfig {
                title: record.value.title,
                date,
                slug,
                summary: record.value.description,
                projects: vec![],
            })
        })
        .collect();

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    Some(posts)
}

/// Fetches a single post by slug (either rkey or path slug).
pub async fn fetch_post(slug: &str) -> Option<Post> {
    use crate::utils::types::ListRecordsResponse;
    use gloo_net::http::Request;
    use pulldown_cmark::{Parser, html};

    let url = format!(
        "https://bsky.social/xrpc/com.atproto.repo.listRecords?repo={}&collection={}",
        ATPROTO_DID, COLLECTION
    );
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }

    let data = resp.json::<ListRecordsResponse>().await.ok()?;

    let doc = data.records.into_iter().find_map(|record| {
        let rkey = record.uri.rsplit('/').next().unwrap_or_default();
        let path_slug = record.value.path.rsplit('/').next().unwrap_or_default();
        let clean = strip_tid_prefix(path_slug);

        if clean == slug || rkey == slug || path_slug == slug {
            Some(record.value)
        } else {
            None
        }
    })?;

    let date = chrono::DateTime::parse_from_rfc3339(&doc.published_at)
        .map(|dt| dt.format("%B %d, %Y").to_string())
        .unwrap_or(doc.published_at.clone());

    let mut markdown = doc.text_content.clone();

    // fallback for Leaflet's block-based content schema
    if markdown.is_empty() {
        if let Some(ref content) = doc.content {
            markdown = extract_leaflet_text(content);
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new(&markdown));

    Some(Post {
        title: doc.title,
        date,
        slug: slug.to_string(),
        summary: doc.description,
        content: html_output,
        projects: vec![],
    })
}

/// strips offprint's 13-char TID prefix (e.g. `3mm7vpa7rt223-slug` → `slug`). Although this is not necessary if I dont use offprint.
fn strip_tid_prefix(slug: &str) -> String {
    if slug.len() > 14
        && slug.as_bytes().get(13) == Some(&b'-')
        && slug[..13].chars().all(|c| c.is_ascii_alphanumeric())
    {
        slug[14..].to_string()
    } else {
        slug.to_string()
    }
}

/// Extracts plaintext from Leaflet's nested block-based content structure.
fn extract_leaflet_text(content: &serde_json::Value) -> String {
    let mut out = String::new();

    let blocks = content
        .get("pages")
        .and_then(|p| p.as_array())
        .and_then(|pages| pages.first())
        .and_then(|page| page.get("blocks"))
        .and_then(|b| b.as_array());

    if let Some(blocks) = blocks {
        for wrapper in blocks {
            if let Some(text) = wrapper
                .get("block")
                .and_then(|b| b.get("plaintext"))
                .and_then(|t| t.as_str())
            {
                out.push_str(text);
                out.push_str("\n\n");
            }
        }
    }

    out
}

struct Frontmatter {
    title: String,
    date: String,
    slug: String,
    summary: String,
}

fn parse_frontmatter(frontmatter: &str) -> Frontmatter {
    let mut fm = Frontmatter {
        title: String::new(),
        date: String::new(),
        slug: String::new(),
        summary: String::new(),
    };

    for line in frontmatter.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim();
            let val = v.trim().trim_matches('"').trim_matches('\'');
            match key {
                "title" => fm.title = val.to_string(),
                "date" => {
                    fm.date = chrono::NaiveDate::parse_from_str(val, "%Y-%m-%d")
                        .map(|d| d.format("%B %d, %Y").to_string())
                        .unwrap_or_else(|_| val.to_string());
                }
                "slug" => fm.slug = val.to_string(),
                "summary" => fm.summary = val.to_string(),
                _ => {}
            }
        }
    }

    fm
}

/// Parses an embedded markdown file (with frontmatter) into a `Post`.
fn parse_embedded_post(raw: &str, default_title: &str, default_slug: &str) -> Post {
    use pulldown_cmark::{Parser, html};

    let parts: Vec<&str> = raw.splitn(3, "---").collect();
    let (frontmatter, body) = if parts.len() >= 3 {
        (parts[1], parts[2])
    } else {
        ("", raw)
    };

    let fm = parse_frontmatter(frontmatter);
    let title = if fm.title.is_empty() {
        default_title.to_owned()
    } else {
        fm.title
    };
    let slug = if fm.slug.is_empty() {
        default_slug.to_owned()
    } else {
        fm.slug
    };

    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new(body));

    Post {
        title,
        date: fm.date,
        slug,
        summary: fm.summary,
        content: html_output,
        projects: vec![],
    }
}

pub async fn fetch_about() -> Option<Post> {
    Some(parse_embedded_post(
        include_str!("../../posts/AboutMe.md"),
        "About Me",
        "aboutme",
    ))
}

pub async fn fetch_projects() -> Option<Post> {
    let raw = include_str!("../../posts/Projects.md");

    let parts: Vec<&str> = raw.splitn(3, "---").collect();
    let frontmatter = if parts.len() >= 3 { parts[1] } else { "" };
    let projects = parse_projects(frontmatter);

    let mut post = parse_embedded_post(raw, "PROJECT ARCHIVES", "projects");
    post.projects = projects;
    Some(post)
}

fn parse_projects(frontmatter: &str) -> Vec<crate::utils::types::ProjectItem> {
    let mut projects = Vec::new();
    let mut current_project = None;

    let mut in_projects = false;
    for line in frontmatter.lines() {
        let trimmed = line.trim();
        if line.starts_with("projects:") {
            in_projects = true;
            continue;
        }

        if in_projects {
            // If we hit a new top-level key (not indented and not a list item), break out
            if !line.starts_with(' ') && !line.starts_with('-') && !trimmed.is_empty() {
                break;
            }

            if trimmed.starts_with("- name:") {
                if let Some(p) = current_project.take() {
                    projects.push(p);
                }
                let name_val = trimmed
                    .trim_start_matches("- name:")
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'');
                current_project = Some(crate::utils::types::ProjectItem {
                    name: name_val.to_string(),
                    desc: String::new(),
                    tech: Vec::new(),
                    link: String::new(),
                    status: String::new(),
                });
            } else if let Some(ref mut p) = current_project {
                if let Some((k, v)) = trimmed.split_once(':') {
                    let val = v.trim().trim_matches('"').trim_matches('\'');
                    match k {
                        "desc" => p.desc = val.to_string(),
                        "link" => p.link = val.to_string(),
                        "status" => p.status = val.to_string(),
                        "tech" => {
                            let clean_val = val.trim_matches('[').trim_matches(']');
                            p.tech = clean_val
                                .split(',')
                                .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if let Some(p) = current_project {
        projects.push(p);
    }

    projects
}
