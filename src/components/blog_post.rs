use crate::content::get_post_image;
use leptos::prelude::*;
use pulldown_cmark::{Event, Parser, Tag, html};
stylance::import_style!(style, "../styles/blog_post.module.css");

#[component]
pub fn BlogPost(title: String, date: String, content: String) -> impl IntoView {
    // 1. Parse Markdown to HTML
    let parser = Parser::new(&content);
    let parser = parser.map(|event| match event {
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            if !dest_url.starts_with("http")
                && !dest_url.starts_with("//")
                && !dest_url.starts_with("data:")
            {
                if let Some(data_uri) = get_post_image(&dest_url) {
                    return Event::Start(Tag::Image {
                        link_type,
                        dest_url: data_uri.into(),
                        title,
                        id,
                    });
                }
            }
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            })
        }
        _ => event,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 2. Render safely using inner_html
    view! {
        <article class=style::article>
            <div class=style::header>
                <h1 class=style::title>{title}</h1>
                <p class=style::date>{date}</p>
            </div>
            <div class=style::content inner_html=html_output></div>
        </article>
    }
}
