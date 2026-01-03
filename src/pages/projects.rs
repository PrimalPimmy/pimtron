use crate::utils::content::fetch_projects;
use leptos::prelude::*;

stylance::import_style!(style, "../styles/projects.module.css");
stylance::import_style!(md_style, "../styles/markdown.module.css");

#[component]
pub fn Projects() -> impl IntoView {
    let projects_resource = LocalResource::new(move || fetch_projects());

    view! {
        <Suspense fallback=move || view! { <div class=style::loading>"LOADING ARMORY MANIFEST..."</div> }>
            {move || {
                projects_resource.get().map(|data| {
                    match data {
                        Some(post) => view! {
                            <div class=style::container>
                                <div class=style::header>
                                    <h1 class=style::title>"// " {post.title}</h1>
                                </div>

                                // Use shared markdown style for the content, but keep layout container logic from projects
                                <div class=md_style::content inner_html=post.content></div>

                                <div class=style::project_grid>
                                    {post.projects.into_iter().map(|proj| view! {

                                        <a href=proj.link target="_blank" class=style::card>
                                            <div class=style::card_header>
                                                <div class=style::card_name>{proj.name.clone()}</div>
                                                <div class=move || if proj.status == "ONLINE" {
                                                    format!("{} {}", style::card_status, style::status_online)
                                                } else {
                                                    style::card_status.to_string()
                                                }>{proj.status.clone()}</div>
                                            </div>
                                            <div class=style::card_desc>{proj.desc.clone()}</div>
                                            <div class=style::tech_stack>
                                                {proj.tech.into_iter().map(|t| view! {
                                                    <span class=style::tech_tag>"[" {t} "]"</span>
                                                }).collect_view()}
                                            </div>
                                        </a>
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_any(),
                        None => view! { <div class=style::loading>"ERROR: MANIFEST NOT FOUND"</div> }.into_any()
                    }
                })
            }}
        </Suspense>
    }
}
