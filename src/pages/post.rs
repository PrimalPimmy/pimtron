use crate::utils::content::fetch_post;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_params_map;
stylance::import_style!(post_style, "../styles/post.module.css");
stylance::import_style!(content_style, "../styles/blog_post.module.css");

#[component]
fn PostContent(title: String, date: String, content: String) -> impl IntoView {
    view! {
        <article class=content_style::article>
            <div class=content_style::header>
                <h1 class=content_style::title>{title}</h1>
                <p class=content_style::date>{date}</p>
            </div>
            <div class=content_style::content inner_html=content></div>
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
        <Suspense fallback=move || view! { <div class=post_style::container><p>"Loading post..."</p></div> }>
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

                    <div class=post_style::container>
                        <div class=post_style::back_link_container>
                            <a href="/blog" class=post_style::back_link>"< Back to Blog"</a>
                        </div>
                        <PostContent title=p.title date=p.date content=p.content />
                    </div>
                }.into_any(),
                Some(None) => view! { 
                    <Title text="Pimtron - Post Not Found" />
                    <div class=post_style::container>
                        <div class=post_style::back_link_container>
                            <a href="/blog" class=post_style::back_link>"< Back to Blog"</a>
                        </div>
                        <p>"Post not found"</p>
                    </div> 
                }.into_any(),
                None => view! { <div class=post_style::container></div> }.into_any() 
            }}
        </Suspense>
    }
}