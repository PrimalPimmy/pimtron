use crate::utils::content::PostConfig;

use leptos::leptos_dom::helpers::set_interval_with_handle;
use leptos::prelude::*;
use leptos_meta::*;
use std::time::Duration;
stylance::import_style!(style, "../styles/home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    let (time_str, set_time_str) = signal("00:00:00".to_string());

    Effect::new(move |_| {
        // Set immediately so it doesn't flash 00:00:00
        {
            let now = js_sys::Date::new_0();
            set_time_str.set(format!(
                "{:02}:{:02}:{:02}",
                now.get_hours(),
                now.get_minutes(),
                now.get_seconds()
            ));
        }

        let handle = set_interval_with_handle(
            move || {
                let now = js_sys::Date::new_0();
                set_time_str.set(format!(
                    "{:02}:{:02}:{:02}",
                    now.get_hours(),
                    now.get_minutes(),
                    now.get_seconds()
                ));
            },
            Duration::from_millis(1000),
        )
        .ok();

        on_cleanup(move || {
            if let Some(h) = handle {
                h.clear();
            }
        });
    });

    view! {
        <Title text="Pimtron" />
        <div class=style::home_container>


            <div class=style::intro_section>
                <p class=style::intro_text>
                    "I " <strong>"build, break, and rebuild " </strong>
                    "things. If there's an idea... well I hope it just doesn't end up stale. "
                    "A particular field does not matter much to me. I will explore the most of what I can in this lifetime."
                </p>
            </div>

            <div class=style::main_content_grid>
                <div class=style::blueprint_panel>
                    <div class=style::blueprint_header>
                        <span>"FIG_1.0: CORE_SYSTEMS"</span>
                        <span>"REV_2026"</span>
                    </div>

                    <div class=style::blueprint_content>
                        <div class=style::diagram_image></div>
                    </div>
                </div>

                // RIGHT: DIGITAL CLOCK
                <div class=style::clock_panel>
                    <div class=style::clock_label>"LOCAL TIME"</div>
                    <div class=style::clock_display>{time_str}</div>
                </div>
            </div>


            <div class=style::bento_grid>

                {move || {
                    let posts_resource = use_context::<LocalResource<Vec<PostConfig>>>();
                    view! {
                        <Suspense fallback=move || view! {
                            // Placeholder tiles while loading
                            <a href="/blog" class=format!("{} {}", style::bento_tile, style::bento_wide)>
                                <div class=style::tile_icon>"✦"</div>
                                <div class=style::tile_content>
                                    <span class=style::tile_eyebrow>"LATEST POST"</span>
                                    <span class=style::tile_title>"Loading..."</span>
                                </div>
                            </a>
                            <a href="/blog" class=format!("{} {}", style::bento_tile, style::bento_tall)>
                                <div class=style::tile_icon>"◈"</div>
                                <div class=style::tile_content>
                                    <span class=style::tile_title>"Recent Posts"</span>
                                    <span class=style::tile_subtitle>"Loading posts..."</span>
                                </div>
                            </a>
                        }>
                            {move || {
                                posts_resource.and_then(|res| res.get()).map(|posts| {
                                    let latest = posts.first().cloned();
                                    let recent: Vec<_> = posts.iter().skip(1).take(3).cloned().collect();

                                    view! {
                                        // LATEST POST — wide tile
                                        {match latest {
                                            Some(post) => {
                                                let dest = format!("/blog/{}", post.slug);
                                                view! {
                                                    <a href=dest class=format!("{} {}", style::bento_tile, style::bento_wide)>
                                                        <div class=style::tile_icon>"✦"</div>
                                                        <div class=style::tile_content>
                                                            <span class=style::tile_eyebrow>"LATEST POST"</span>
                                                            <span class=style::tile_title>{post.title}</span>
                                                            <span class=style::tile_date>{post.date}</span>
                                                            <span class=style::tile_subtitle>{post.summary}</span>
                                                        </div>
                                                    </a>
                                                }.into_any()
                                            },
                                            None => view! {
                                                <a href="/blog" class=format!("{} {}", style::bento_tile, style::bento_wide)>
                                                    <div class=style::tile_icon>"✦"</div>
                                                    <div class=style::tile_content>
                                                        <span class=style::tile_eyebrow>"LATEST POST"</span>
                                                        <span class=style::tile_title>"No posts yet"</span>
                                                    </div>
                                                </a>
                                            }.into_any()
                                        }}

                                        <div class=format!("{} {}", style::bento_tile, style::bento_tall)>
                                            <div class=style::tile_icon>"◈"</div>
                                            <div class=style::tile_content>
                                                <span class=style::tile_title>"Recent"</span>
                                                <div class=style::recent_list>
                                                    {recent.into_iter().map(|p| {
                                                        let dest = format!("/blog/{}", p.slug);
                                                        view! {
                                                            <a href=dest class=style::recent_item>
                                                                <span class=style::recent_item_title>{p.title}</span>
                                                                <span class=style::recent_item_date>{p.date}</span>
                                                            </a>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })
                            }}
                        </Suspense>
                    }
                }}


                <a href="/blog" class=format!("{} {}", style::bento_tile, style::bento_medium)>
                    <div class=style::tile_icon>"⊞"</div>
                    <div class=style::tile_content>
                        <span class=style::tile_title>"All Posts"</span>
                        <span class=style::tile_subtitle>"Read all transmissions"</span>
                    </div>
                </a>
                <a href="/projects" class=format!("{} {}", style::bento_tile, style::bento_medium)>
                    <div class=style::tile_icon>"⟐"</div>
                    <div class=style::tile_content>
                        <span class=style::tile_title>"Projects"</span>
                        <span class=style::tile_subtitle>"Artifact database & tools"</span>
                    </div>
                </a>


                <a href="/about" class=format!("{} {}", style::bento_tile, style::bento_medium)>
                    <div class=style::tile_icon>"⊕"</div>
                    <div class=style::tile_content>
                        <span class=style::tile_title>"About me"</span>
                        <span class=style::tile_subtitle>"User profile & bio"</span>
                    </div>
                </a>
                <a href="/contact" class=format!("{} {}", style::bento_tile, style::bento_medium)>
                    <div class=style::tile_icon>"⊙"</div>
                    <div class=style::tile_content>
                        <span class=style::tile_title>"Contact"</span>
                        <span class=style::tile_subtitle>"Establish comms link"</span>
                    </div>
                </a>
            </div>

        </div>
    }
}
