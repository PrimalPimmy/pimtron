use crate::utils::state::AboutResource;
use crate::components::navbar::Navbar;
use leptos::prelude::*;

stylance::import_style!(article_style, "../styles/article.module.css");

stylance::import_style!(
    #[allow(unused)]
    common,
    "../styles/common.module.css"
);

#[component]
pub fn About() -> impl IntoView {
    let about_resource = use_context::<AboutResource>()
        .expect("AboutResource should be provided in App")
        .0;

    view! {
        <Navbar />
        <Suspense fallback=move || view! { <div class="loading">"INITIALIZING UPLINK..."</div> }>
            {move || {
                about_resource.get().map(|data| {
                    match data {
                        Some(post) => view! {
                            <div class=common::container>
                                <div class=article_style::header>
                                    <h1 class=common::page_title>{post.title}</h1>
                                    <div class=article_style::date>"ID: " {post.slug} " // LAST_UPDATE: " {post.date}</div>
                                </div>
                                <div class=article_style::content inner_html=post.content></div>
                            </div>
                        }.into_any(),
                        None => view! { <div class="loading">"ERROR: DATA CORRUPTED"</div> }.into_any()
                    }
                })
            }}
        </Suspense>
    }
}
