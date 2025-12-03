use leptos::prelude::*;
use crate::content::get_all_posts;
stylance::import_style!(style, "../styles/blog.module.css");

#[component]
pub fn Blog() -> impl IntoView {
    let posts = get_all_posts();

    view! {
        <div class=style::container>
            <h1 class=style::title>"Latest Blog Posts"</h1>
            <div class=style::post_list>
                {posts.into_iter().map(|post| {
                    view! {
                        <a href=format!("/blog/{}", post.slug) class=style::article_card>
                            <h2 class=style::article_title>{post.title}</h2>
                            <p class=style::date>{post.date.clone()}</p>
                            <p class=style::summary>{post.summary}</p>
                        </a>
                    }
                }).collect_view()}
            </div>
            <div class=style::back_link_container>
                <a href="/" class=style::back_link>"< Back to Home"</a>
            </div>
        </div>
    }
}
