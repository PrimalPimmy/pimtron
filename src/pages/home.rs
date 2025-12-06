use leptos::prelude::*;
use leptos_meta::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
stylance::import_style!(style, "../styles/home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    let (webgl_active, set_webgl_active) = signal(false);
    let (webgpu_active, set_webgpu_active) = signal(false);

    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Check WebGL
        if let Ok(canvas) = document.create_element("canvas")
            && let Ok(canvas_el) = canvas.dyn_into::<HtmlCanvasElement>() {
                // Try webgl2 first, then webgl
                let gl2 = canvas_el.get_context("webgl2");
                let gl1 = canvas_el.get_context("webgl");
                
                if let Ok(Some(_)) = gl2 {
                    set_webgl_active.set(true);
                } else if let Ok(Some(_)) = gl1 {
                    set_webgl_active.set(true);
                }
            }

        // Check WebGPU
        let navigator = window.navigator();
        
        leptos::task::spawn_local(async move {
            let navigator_js: &wasm_bindgen::JsValue = navigator.as_ref();
            if let Ok(gpu_val) = js_sys::Reflect::get(navigator_js, &"gpu".into())
                && !gpu_val.is_undefined() && !gpu_val.is_null() {
                     // We use Reflect to call requestAdapter to avoid strict type dependencies 
                     // that might be missing or named differently in web-sys versions.
                     // gpu.requestAdapter() -> Promise<GPUAdapter?>
                     
                     let request_adapter_key = wasm_bindgen::JsValue::from_str("requestAdapter");
                     if let Ok(request_adapter_fn_val) = js_sys::Reflect::get(&gpu_val, &request_adapter_key)
                         && let Ok(request_adapter_fn) = request_adapter_fn_val.dyn_into::<js_sys::Function>()
                             && let Ok(promise_val) = request_adapter_fn.call0(&gpu_val) {
                                 let promise = promise_val.unchecked_into::<js_sys::Promise>();
                                 if let Ok(adapter) = wasm_bindgen_futures::JsFuture::from(promise).await
                                     && !adapter.is_null() && !adapter.is_undefined() {
                                         set_webgpu_active.set(true);
                                     }
                             }
                }
        });
    });

    view! {
        <Title text="Pimtron" />
        <div class=style::home_container>
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