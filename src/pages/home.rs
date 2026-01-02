use crate::utils::gpu::{check_webgl_support, check_webgpu_support, track_fps};

use leptos::html::Div;
use leptos::leptos_dom::helpers::set_interval_with_handle;
use leptos::prelude::*;
use leptos_meta::*;
use std::time::Duration;
stylance::import_style!(style, "../styles/home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    let (webgl_active, set_webgl_active) = signal(false);
    let (webgpu_active, set_webgpu_active) = signal(false);
    let (fps, set_fps) = signal(0);
    let (time_str, set_time_str) = signal(String::new());
    let container_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        // Check WebGL
        if check_webgl_support() {
            set_webgl_active.set(true);
        }

        // Check WebGPU
        leptos::task::spawn_local(async move {
            if check_webgpu_support().await {
                set_webgpu_active.set(true);
            }
        });

        track_fps(set_fps);

        // Earth Time Clock
        let handle = set_interval_with_handle(
            move || {
                let now = js_sys::Date::new_0();
                let hours = now.get_hours();
                let minutes = now.get_minutes();
                let seconds = now.get_seconds();
                let milliseconds = now.get_milliseconds();

                let formatted = format!(
                    "{:02}:{:02}:{:02}:{:02}",
                    hours,
                    minutes,
                    seconds,
                    milliseconds / 10
                );
                set_time_str.set(formatted);
            },
            Duration::from_millis(33), // ~30fps update for clock
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
        <div class=style::home_container node_ref=container_ref>
            <div class=style::page_corner_tl></div>
            <div class=style::page_corner_tr></div>
            <div class=style::page_corner_bl></div>
            <div class=style::page_corner_br></div>

            // Earth Time Clock
            <div class=style::earth_clock>
                <span class=style::earth_label>"EARTH TIME"</span>
                <span class=style::earth_time>{time_str}</span>
            </div>

            <div class=style::hud_panel>
                <div class=style::hud_group>
                    <div class=style::hud_label>"SYS DIAG"</div>

                    // WebGL Status
                    <div class=style::hud_row>
                        <span class=move || if webgl_active.get() {
                            format!("{} {}", style::hud_status_indicator, style::hud_status_active)
                        } else {
                            style::hud_status_indicator.to_string()
                        }></span>
                        <span>"WEBGL"</span>
                        <div class=style::hud_spacer></div>
                        <span class=style::hud_value>{move || if webgl_active.get() { "ON" } else { "OFF" }}</span>
                    </div>

                    // WebGPU Status
                    <div class=style::hud_row>
                        <span class=move || if webgpu_active.get() {
                            format!("{} {}", style::hud_status_indicator, style::hud_status_active)
                        } else {
                            style::hud_status_indicator.to_string()
                        }></span>
                        <span>"WEBGPU"</span>
                        <div class=style::hud_spacer></div>
                        <span class=style::hud_value>{move || if webgpu_active.get() { "ON" } else { "OFF" }}</span>
                    </div>

                    // FPS Counter
                    <div class=style::hud_row>
                        <span class=format!("{} {}", style::hud_status_indicator, style::hud_status_active)></span>
                        <span>"FPS"</span>
                        <div class=style::hud_bar_container>
                             <div class=style::hud_bar_fill style:width=move || format!("{}%", (fps.get() as f32 / 144.0 * 100.0).min(100.0)) ></div>
                        </div>
                        <span class=style::hud_value>{fps}</span>
                    </div>
                </div>
            </div>

            // Main Content
            <div class=style::content_wrapper>
                <h1 class=style::hero_title>
                    "Hey, I'm Prashant a.k.a Pimtron/Pimmy"
                </h1>
                <p class=style::hero_subtitle>
                    "I'm a " <span class=style::highlight>"Software Developer"</span>
                    ". I love to talk about the wide spectrum in Tech."
                </p>
                <div class=style::actions>
                    <a href="/blog" class=style::btn_primary>"Read my blog"</a>
                </div>
            </div>

            // Diagram Section
            <div class=style::diagram_container>
                 <div class=style::diagram_image></div>
            </div>
        </div>
    }
}
