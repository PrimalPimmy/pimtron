use crate::components::navbar::Navbar;
use crate::components::sticky_note::StickyNote;
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
    let (time_str, set_time_str) = signal("00:00:00".to_string());

    let container_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        if check_webgl_support() {
            set_webgl_active.set(true);
        }
        leptos::task::spawn_local(async move {
            if check_webgpu_support().await {
                set_webgpu_active.set(true);
            }
        });
        track_fps(set_fps);

        let handle = set_interval_with_handle(
            move || {
                let now = js_sys::Date::new_0();
                let f = format!(
                    "{:02}:{:02}:{:02}",
                    now.get_hours(),
                    now.get_minutes(),
                    now.get_seconds()
                );
                set_time_str.set(f);
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
        <div class=style::home_container node_ref=container_ref>

            // --- SECTION 1: MASTHEAD ---
            <div class=style::masthead_container>
                <h1 class=style::giant_title>"PIMTRON"</h1>

                // Navbar is now just the link row, placed below header as requested
                <div class=style::nav_row>
                    <Navbar />
                </div>
            </div>

            // --- SECTION 2: SPLIT CONTENT ---
            <div class=style::main_content_grid>
                // STICKY NOTES LAYER (Absolute)
                <StickyNote initial_x={-240.0} initial_y={200.0}>
                    "Don't forget to check system logs."
                    <br/>
                    "P.S; These notes can be dragged with a mouse."
                </StickyNote>

                <StickyNote initial_x={-125.0} initial_y={473.0}>
                    "Review Diagram FIG_1.0"
                    <br/>
                    "Looks a bit small."
                </StickyNote>

                // LEFT: BLUEPRINT PANEL
                <div class=style::blueprint_panel>
                    <div class=style::blueprint_header>
                        <span>"FIG_1.0: CORE_SYSTEMS"</span>
                        <span>"REV_2026"</span>
                    </div>

                    <div class=style::blueprint_content>
                        <div class=style::diagram_image></div>
                    </div>
                </div>

                // RIGHT: INSTRUMENT PANEL
                <div class=style::instrument_panel>

                    // VOLTMETER GAUGE
                    <div class=style::voltmeter_container>
                        <div class=style::gauge_label>"SYSTEM LOAD"</div>
                        <div class=style::gauge_wrapper>
                            <div class=style::gauge_dial>
                                // Needle is positioned at center with transform-origin: top center
                                // Base rotation is 180deg (pointing up). Add -70 to +70 to sweep the semicircle
                                <div class=style::gauge_needle style=move || {
                                    let clamped_fps = fps.get().min(265) as f64;
                                    // Map 0-265 FPS: 90deg (horizontal left) to 270deg (horizontal right)
                                    let rotation = 90.0 + (clamped_fps / 265.0 * 180.0);
                                    format!("transform: rotate({}deg); transition: transform 0.2s cubic-bezier(0.1, 0.7, 1.0, 0.1);", rotation)
                                }></div>
                            </div>
                        </div>

                        <div class=style::readout_row>
                            <div class=style::digital_group>
                                <span class=style::digital_label>"FPS"</span>
                                <div class=style::digital_readout>{fps}</div>
                            </div>
                            <div class=style::digital_group>
                                <span class=style::digital_label>"TIME"</span>
                                <div class=style::digital_readout>{time_str}</div>
                            </div>
                        </div>
                    </div>

                    // STATUS MODULES
                    <div class=style::voltmeter_container>
                        <div class=style::gauge_label>"ACTIVE PROTOCOLS"</div>
                        <div style="display:flex; flex-direction:column; gap:10px; width:100%; margin-top:10px;">
                            <div style="display:flex; justify-content:space-between; font-family:var(--font-mono);">
                                <span>"WEBGL_RENDERER"</span>
                                <span style="font-weight:bold; color:var(--color-primary)">
                                    {move || if webgl_active.get() { "ONLINE" } else { "OFFLINE" }}
                                </span>
                            </div>
                            <div style="display:flex; justify-content:space-between; font-family:var(--font-mono);">
                                <span>"WEBGPU_COMPUTE"</span>
                                <span style="font-weight:bold; color:var(--color-primary)">
                                    {move || if webgpu_active.get() { "ONLINE" } else { "OFFLINE" }}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // --- SECTION 3: INTRO ---
            <div class=style::intro_section>
                <p class=style::intro_text>
                    "Hey, I'm Prashant, a.k.a Pimtron/Pimmy. I love to explore about the wide spectrum in tech! I have worked in the "
                    <strong>"Security"</strong>
                    " field, especially in the "
                    <strong>"Cloud Native"</strong>
                    " side. I have also been exploring "
                    <strong>"Graphics Programming"</strong>
                    " and "
                    <strong>"Distributed systems"</strong>
                    ". So yeah, I do want to see if I can have the knowledge in many fields xD."
                </p>
            </div>

            // --- SECTION 4: CONTROL DECK ---
            <div class=style::control_deck>
                <a href="/projects" class=style::control_module>
                    <span class=style::module_label>"PROJECTS"</span>
                    <span class=style::module_status>"View artifact database & tools >>"</span>
                </a>
                <a href="/about" class=style::control_module>
                    <span class=style::module_label>"ABOUT"</span>
                    <span class=style::module_status>"Access user profile & bio >>"</span>
                </a>
                <a href="/blog" class=style::control_module>
                    <span class=style::module_label>"BLOG"</span>
                    <span class=style::module_status>"Read transmission logs >>"</span>
                </a>
                 <a href="mailto:contact@pimtron.com" class=style::control_module>
                    <span class=style::module_label>"CONTACT"</span>
                    <span class=style::module_status>"Establish comms link >>"</span>
                </a>
            </div>

        </div>
    }
}
