use crate::components::blog_post::BlogPost;
use crate::utils::content::fetch_post;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_params_map;
stylance::import_style!(style, "../styles/post.module.css");

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
        <Suspense fallback=move || view! { <div class=style::container><p>"Loading post..."</p></div> }>
            {move || match post_resource.get() {
                Some(Some(p)) => view! {
                    <Title text=format!("Pimtron - {}", p.title) />
                    <div class=style::container>
                        <div class=style::back_link_container>
                            <a href="/blog" class=style::back_link>"< Back to Blog"</a>
                        </div>
                        <BlogPost title=p.title date=p.date content=p.content />
                    </div>
                }.into_any(),
                Some(None) => view! { 
                    <Title text="Pimtron - Post Not Found" />
                    <div class=style::container>
                        <div class=style::back_link_container>
                            <a href="/blog" class=style::back_link>"< Back to Blog"</a>
                        </div>
                        <p>"Post not found"</p>
                    </div> 
                }.into_any(),
                None => view! { <div class=style::container></div> }.into_any() // Suspense handles the fallback, this is just to satisfy types if accessed before ready
            }}
        </Suspense>
    }
}
