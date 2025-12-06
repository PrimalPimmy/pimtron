use crate::utils::gpu::{check_webgl_support, check_webgpu_support};

use leptos::html::Div;
use leptos::prelude::*;
use leptos_meta::*;
stylance::import_style!(style, "../styles/home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    let (webgl_active, set_webgl_active) = signal(false);
    let (webgpu_active, set_webgpu_active) = signal(false);
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


    });

    view! {
        <Title text="Pimtron" />
        <div class=style::home_container node_ref=container_ref>
            <div class=style::gpu_status_container>
                <div class=style::gpu_status_item>
                    <span>"WebGL"</span>
                    <span class=move || if webgl_active.get() { style::gpu_status_active } else { style::gpu_status_inactive }>
                        {move || if webgl_active.get() { "ON" } else { "OFF" }}
                    </span>
                </div>
                <div class=style::gpu_status_item>
                    <span>"WebGPU"</span>
                    <span class=move || if webgpu_active.get() { style::gpu_status_active } else { style::gpu_status_inactive }>
                        {move || if webgpu_active.get() { "ON" } else { "OFF" }}
                    </span>
                </div>
            </div>

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
        </div>
    }
}
