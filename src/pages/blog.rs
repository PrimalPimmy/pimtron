use crate::utils::content::{self};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_navigate;
use std::time::Duration;
stylance::import_style!(style, "../styles/blog.module.css");

#[component]
pub fn Blog() -> impl IntoView {
    let posts = content::get_all_posts(); // Call the synchronous function
    
    let navigate = use_navigate();

    view! {
        <Title text="Pimtron - Blog" />
        <div class=style::container>
            <h1 class=style::title>"Latest Blog Posts"</h1>
            <div class=style::post_list>
                // Suspense is no longer needed here as data is synchronous
                {if posts.is_empty() {
                    view! { <p>"No posts found."</p> }.into_any()
                } else {
                    posts.into_iter().map(|post| {
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
                    }).collect_view().into_any()
                }}
            </div>
            <div class=style::back_link_container>
                <a href="/" class=style::back_link>"< Back to Home"</a>
            </div>
        </div>
    }
}
