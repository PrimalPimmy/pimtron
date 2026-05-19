pub use crate::utils::types::{Post, PostConfig};

const ATPROTO_DID: &str = "did:plc:kdcc475mwmfd7ehlvhqxgrqr";
const COLLECTION: &str = "site.standard.document";

pub async fn fetch_all_posts() -> Vec<PostConfig> {
    use crate::utils::types::ListRecordsResponse;
    use gloo_net::http::Request;

    let url = format!(
        "https://bsky.social/xrpc/com.atproto.repo.listRecords?repo={}&collection={}",
        ATPROTO_DID, COLLECTION
    );
    let response = Request::get(&url).send().await;

    if let Ok(resp) = response {
        if resp.ok() {
            if let Ok(data) = resp.json::<ListRecordsResponse>().await {
                let mut posts = Vec::new();
                for record in data.records {
                    let doc = record.value;
                    let rkey = record.uri.split('/').last().unwrap_or("").to_string();

                    let mut slug = doc.path.split('/').last().unwrap_or("").to_string();

                    if slug.is_empty() {
                        slug = rkey.clone();
                    }

                    // Skip special pages
                    if rkey == "aboutme"
                        || rkey == "projects"
                        || slug.to_lowercase() == "aboutme"
                        || slug.to_lowercase() == "projects"
                    {
                        continue;
                    }

                    let date = chrono::DateTime::parse_from_rfc3339(&doc.published_at)
                        .map(|dt| dt.format("%B %d, %Y").to_string())
                        .unwrap_or(doc.published_at);

                    posts.push(PostConfig {
                        title: doc.title,
                        date,
                        slug,
                        summary: doc.description,
                        projects: vec![],
                    });
                }
                // Sort posts by date descending
                posts.sort_by(|a, b| b.date.cmp(&a.date));
                return posts;
            }
        }
    }
    Vec::new()
}

/// Fetches a single post by slug (either rkey or path slug).
pub async fn fetch_post(slug: &str) -> Option<Post> {
    use crate::utils::types::ListRecordsResponse;
    use gloo_net::http::Request;
    use pulldown_cmark::{Parser, html};

    // instead of getRecord (which strictly requires the rkey), we fetch listRecords
    // and find the one that matches our desired slug in the path
    let url = format!(
        "https://bsky.social/xrpc/com.atproto.repo.listRecords?repo={}&collection={}",
        ATPROTO_DID, COLLECTION
    );
    let response = Request::get(&url).send().await.ok()?;

    if !response.ok() {
        return None;
    }

    let data = response.json::<ListRecordsResponse>().await.ok()?;

    let mut found_doc = None;
    for record in data.records {
        let rkey = record.uri.split('/').last().unwrap_or("");
        let path_slug = record.value.path.split('/').last().unwrap_or("");

        let mut clean_path_slug = path_slug.to_string();
        if clean_path_slug.len() > 14 && clean_path_slug.chars().nth(13) == Some('-') {
            let is_tid = clean_path_slug
                .chars()
                .take(13)
                .all(|c| c.is_ascii_alphanumeric());
            if is_tid {
                clean_path_slug = clean_path_slug[14..].to_string();
            }
        }

        if clean_path_slug == slug || rkey == slug || path_slug == slug {
            found_doc = Some(record.value);
            break;
        }
    }

    let doc = found_doc?;

    let date = chrono::DateTime::parse_from_rfc3339(&doc.published_at)
        .map(|dt| dt.format("%B %d, %Y").to_string())
        .unwrap_or(doc.published_at.clone());

    // Standard textContent fallback
    let mut markdown = doc.text_content.clone();

    // Fallback for Leaflet which uses a custom block-based schema instead of textContent
    if markdown.is_empty() {
        if let Some(content_val) = &doc.content {
            if let Some(pages) = content_val.get("pages").and_then(|p| p.as_array()) {
                if let Some(first_page) = pages.first() {
                    if let Some(blocks) = first_page.get("blocks").and_then(|b| b.as_array()) {
                        for block_wrapper in blocks {
                            if let Some(block) = block_wrapper.get("block") {
                                if let Some(plaintext) =
                                    block.get("plaintext").and_then(|t| t.as_str())
                                {
                                    markdown.push_str(plaintext);
                                    markdown.push_str("\n\n");
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut html_output = String::new();
    let parser = Parser::new(&markdown);
    html::push_html(&mut html_output, parser);

    Some(Post {
        title: doc.title,
        date,
        slug: slug.to_string(),
        summary: doc.description,
        content: html_output,
        projects: vec![],
    })
}

fn parse_frontmatter(frontmatter: &str) -> (String, String, String, String) {
    let mut title = String::new();
    let mut date = String::new();
    let mut slug = String::new();
    let mut summary = String::new();

    for line in frontmatter.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim();
            let val = v.trim().trim_matches('"').trim_matches('\'');
            match key {
                "title" => title = val.to_string(),
                "date" => {
                    date = chrono::NaiveDate::parse_from_str(val, "%Y-%m-%d")
                        .map(|d| d.format("%B %d, %Y").to_string())
                        .unwrap_or_else(|_| val.to_string());
                }
                "slug" => slug = val.to_string(),
                "summary" => summary = val.to_string(),
                _ => {}
            }
        }
    }
    (title, date, slug, summary)
}

pub async fn fetch_about() -> Option<Post> {
    use pulldown_cmark::{Parser, html};
    let raw = include_str!("../../posts/AboutMe.md");

    let parts: Vec<&str> = raw.splitn(3, "---").collect();
    let (frontmatter, body) = if parts.len() >= 3 {
        (parts[1], parts[2])
    } else {
        ("", raw)
    };

    let (mut title, date, mut slug, summary) = parse_frontmatter(frontmatter);
    if title.is_empty() {
        title = "About Me".to_string();
    }
    if slug.is_empty() {
        slug = "aboutme".to_string();
    }

    let mut html_output = String::new();
    let parser = Parser::new(body);
    html::push_html(&mut html_output, parser);

    Some(Post {
        title,
        date,
        slug,
        summary,
        content: html_output,
        projects: vec![],
    })
}

pub async fn fetch_projects() -> Option<Post> {
    use pulldown_cmark::{Parser, html};
    let raw = include_str!("../../posts/Projects.md");

    let parts: Vec<&str> = raw.splitn(3, "---").collect();
    let (frontmatter, body) = if parts.len() >= 3 {
        (parts[1], parts[2])
    } else {
        ("", raw)
    };

    let (title, date, slug, summary) = parse_frontmatter(frontmatter);
    let projects = parse_projects(frontmatter);

    // Fallbacks if missing
    let title = if title.is_empty() {
        "PROJECT ARCHIVES".to_string()
    } else {
        title
    };
    let slug = if slug.is_empty() {
        "projects".to_string()
    } else {
        slug
    };

    let mut html_output = String::new();
    let parser = Parser::new(body);
    html::push_html(&mut html_output, parser);

    Some(Post {
        title,
        date,
        slug,
        summary,
        content: html_output,
        projects,
    })
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
