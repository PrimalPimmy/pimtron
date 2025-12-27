use leptos::prelude::*;
use pulldown_cmark::{Parser, html};
stylance::import_style!(style, "../styles/blog_post.module.css");

#[component]
pub fn BlogPost(title: String, date: String, content: String) -> impl IntoView {
    // 1. Parse Markdown to HTML
    let parser = Parser::new(&content);
    
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
