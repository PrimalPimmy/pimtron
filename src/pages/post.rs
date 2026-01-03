use crate::utils::content::fetch_post;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_params_map;
stylance::import_style!(style, "../styles/markdown.module.css");

#[component]
fn PostContent(title: String, date: String, content: String) -> impl IntoView {
    view! {
        <article>
            <header class=style::header>
                <h1 class=style::title>{title}</h1>
                <div class=style::date>{date}</div>
            </header>
            <div class=style::content inner_html=content></div>
        </article>
    }
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();

    let post_resource = LocalResource::new(move || async move {
        let params_map = params.get();
        let slug = params_map.get("slug").unwrap_or_default();
        if slug.is_empty() {
            return None;
        }
        fetch_post(&slug).await
    });

    view! {
        <Suspense fallback=move || view! { <div class=style::markdown_container><p>"Loading post..."</p></div> }>
            {move || match post_resource.get() {
                Some(Some(p)) => view! {
                    <Title text=format!("Pimtron - {}", p.title) />
                    <Meta name="description" content=p.summary.clone() />
                    
                    // Open Graph
                    <Meta property="og:title" content=p.title.clone() />
                    <Meta property="og:description" content=p.summary.clone() />
                    <Meta property="og:type" content="article" />
                    
                    // Twitter
                    <Meta name="twitter:title" content=p.title.clone() />
                    <Meta name="twitter:description" content=p.summary.clone() />

                    <div class=style::markdown_container>
                        <div class=style::back_link_container>
                            <a href="/blog" class=style::back_link>"< Back to Blog"</a>
                        </div>
                        <PostContent title=p.title date=p.date content=p.content />
                    </div>
                }.into_any(),
                Some(None) => view! { 
                    <Title text="Pimtron - Post Not Found" />
                    <div class=style::markdown_container>
                        <div class=style::back_link_container>
                            <a href="/blog" class=style::back_link>"< Back to Blog"</a>
                        </div>
                        <p>"Post not found"</p>
                    </div> 
                }.into_any(),
                None => view! { <div class=style::markdown_container></div> }.into_any() 
            }}
        </Suspense>
    }
}