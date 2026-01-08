use crate::utils::content::PostConfig;
use leptos::prelude::*;
use leptos_meta::*;
stylance::import_style!(style, "../styles/blog.module.css");
stylance::import_style!(
    #[allow(unused)]
    common,
    "../styles/common.module.css"
);

#[component]
pub fn BlogList() -> impl IntoView {
    // Consume the eagerly-loaded posts from context
    let posts_resource = use_context::<LocalResource<Vec<PostConfig>>>()
        .expect("Post resource should be provided in App");

    view! {
        <Title text="Pimtron - Blog" />
        <div class=common::container>
            <h1 class=style::title>"Latest Blog Posts"</h1>
            <div class=style::post_list>
                <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
                    {move || {
                        posts_resource.get().map(|posts| {
                            if posts.is_empty() {
                                view! { <p>"No posts found."</p> }.into_any()
                            } else {
                                posts.into_iter().map(|post| {
                                    let dest = format!("/blog/{}", post.slug);
                                    view! {
                                        <a href=dest class=style::article_card>
                                            <h2 class=style::article_title>{post.title}</h2>
                                            <p class=style::date>{post.date.clone()}</p>
                                            <p class=style::summary>{post.summary}</p>
                                        </a>
                                    }
                                }).collect_view().into_any()
                            }
                        })
                    }}
                </Suspense>
            </div>
            <div class=common::back_link_container>
                <a href="/" class=common::back_link>"< Back to Home"</a>
            </div>
        </div>
    }
}
