use crate::content::get_all_posts;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::time::Duration;
stylance::import_style!(style, "../styles/blog.module.css");

#[component]
pub fn Blog() -> impl IntoView {
    let posts = get_all_posts();
    let navigate = use_navigate();

    view! {
        <div class=style::container>
            <h1 class=style::title>"Latest Blog Posts"</h1>
            <div class=style::post_list>
                {posts.into_iter().map(|post| {
                    let navigate = navigate.clone();
                    let dest = format!("/blog/{}", post.slug);
                    view! {
                        <a href=dest.clone() class=style::article_card on:click=move |e| {
                            e.prevent_default();
                            let navigate = navigate.clone();
                            let dest = dest.clone();
                            set_timeout(move || {
                                navigate(&dest, Default::default());
                            }, Duration::from_millis(200));
                        }>
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
