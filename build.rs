use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostConfig {
    title: String,
    date: String,
    slug: String,
    summary: String,
}

fn main() {
    println!("cargo:rerun-if-changed=posts");

    let posts_dir = "posts";
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let output_file = Path::new(&out_dir).join("posts.json");
    
    let mut posts = Vec::new();
    let matter = Matter::<YAML>::new();

    if let Ok(entries) = fs::read_dir(posts_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                // Rerun if any specific markdown file changes
                println!("cargo:rerun-if-changed={}", path.display());

                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(parsed) = matter.parse::<gray_matter::Pod>(&content) {
                        if let Some(data) = parsed.data {
                            if let Ok(config) = data.deserialize::<PostConfig>() {
                                posts.push(config);
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort by date (descending)
    posts.sort_by(|a, b| {
        let to_sortable = |date: &str| -> String {
            let parts: Vec<&str> = date.split('-').collect();
            if parts.len() == 3 {
                format!("{}-{}-{}", parts[2], parts[1], parts[0])
            } else {
                date.to_string()
            }
        };
        to_sortable(&b.date).cmp(&to_sortable(&a.date))
    });

    let json = serde_json::to_string_pretty(&posts).expect("Failed to serialize posts");
    fs::write(output_file, json).expect("Failed to write posts.json to OUT_DIR");
}
