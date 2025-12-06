use crate::components::blog_post::BlogPost;
use crate::utils::content::get_post_by_slug;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_params_map;
stylance::import_style!(style, "../styles/post.module.css");

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();

    // lookup implementation for posts based on slug
    let post = move || {
        let params_map = params.get();
        let slug = params_map.get("slug").unwrap_or_default();
        get_post_by_slug(&slug)
    };

    view! {
        {move || match post() {
            Some(p) => view! {
                <Title text=format!("Pimtron - {}", p.title) />
                <div class=style::container>
                    <div class=style::back_link_container>
                        <a href="/blog" class=style::back_link>"< Back to Blog"</a>
                    </div>
                    <BlogPost title=p.title date=p.date content=p.content />
                </div>
            }.into_any(),
            None => view! { 
                <Title text="Pimtron - Post Not Found" />
                <div class=style::container><p>"Post not found"</p></div> 
            }.into_any()
        }}
    }
}
