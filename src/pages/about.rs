use leptos::prelude::*;
use crate::utils::content::fetch_about;

stylance::import_style!(article_style, "../styles/article.module.css");
stylance::import_style!(md_style, "../styles/markdown.module.css");

#[component]
pub fn About() -> impl IntoView {
    let about_resource = LocalResource::new(fetch_about);

    view! {
        <Suspense fallback=move || view! { <div class="loading">"INITIALIZING UPLINK..."</div> }>
            {move || {
                about_resource.get().map(|data| {
                    match data {
                        Some(post) => view! {
                            <div class=article_style::container>
                                <div class=article_style::header>
                                    <h1 class=article_style::title>{post.title}</h1>
                                    <div class=article_style::date>"ID: " {post.slug} " // LAST_UPDATE: " {post.date}</div>
                                </div>
                                <div class=md_style::content inner_html=post.content></div>
                            </div>
                        }.into_any(),
                        None => view! { <div class="loading">"ERROR: DATA CORRUPTED"</div> }.into_any()
                    }
                })
            }}
        </Suspense>
    }
}
