use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use leptos::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

pub fn check_webgl_support() -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return false,
    };

    if let Ok(canvas) = document.create_element("canvas")
        && let Ok(canvas_el) = canvas.dyn_into::<HtmlCanvasElement>()
    {
        // Try webgl2 first, then webgl
        let gl2 = canvas_el.get_context("webgl2");
        let gl1 = canvas_el.get_context("webgl");

        if let Ok(Some(_)) = gl2 {
            return true;
        } else if let Ok(Some(_)) = gl1 {
            return true;
        }
    }
    false
}

pub async fn check_webgpu_support() -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };
    let navigator = window.navigator();

    let navigator_js: &wasm_bindgen::JsValue = navigator.as_ref();
    if let Ok(gpu_val) = js_sys::Reflect::get(navigator_js, &"gpu".into())
        && !gpu_val.is_undefined()
        && !gpu_val.is_null()
    {
        let request_adapter_key = wasm_bindgen::JsValue::from_str("requestAdapter");
        if let Ok(request_adapter_fn_val) = js_sys::Reflect::get(&gpu_val, &request_adapter_key)
            && let Ok(request_adapter_fn) = request_adapter_fn_val.dyn_into::<js_sys::Function>()
            && let Ok(promise_val) = request_adapter_fn.call0(&gpu_val)
        {
            let promise = promise_val.unchecked_into::<js_sys::Promise>();
            if let Ok(adapter) = wasm_bindgen_futures::JsFuture::from(promise).await
                && !adapter.is_null()
                && !adapter.is_undefined()
            {
                return true;
            }
        }
    }
    false
}

pub fn track_fps(set_fps: WriteSignal<i32>) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let performance = match window.performance() {
        Some(p) => p,
        None => return,
    };

    let last_time = Rc::new(RefCell::new(performance.now()));
    let frame_count = Rc::new(RefCell::new(0));
    // Use Arc<AtomicI32> for thread-safety (required by on_cleanup in Leptos 0.8+)
    // Initialize with 0 (invalid ID)
    let handle = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();
    
    let window_clone = window.clone();
    let performance_clone = performance.clone();
    let handle_clone = handle.clone();

    let loop_fn = move || {
        let now = performance_clone.now();
        *frame_count.borrow_mut() += 1;

        let delta = now - *last_time.borrow();
        if delta >= 1000.0 {
            set_fps.set(*frame_count.borrow());
            *frame_count.borrow_mut() = 0;
            *last_time.borrow_mut() = now;
        }

        if let Some(cb) = g.borrow().as_ref() {
            let id = window_clone.request_animation_frame(cb.as_ref().unchecked_ref()).unwrap_or(0);
            handle_clone.store(id, std::sync::atomic::Ordering::Relaxed);
        }
    };

    *f.borrow_mut() = Some(Closure::wrap(Box::new(loop_fn) as Box<dyn FnMut()>));

    if let Some(cb) = f.borrow().as_ref() {
        let id = window.request_animation_frame(cb.as_ref().unchecked_ref()).unwrap_or(0);
        handle.store(id, std::sync::atomic::Ordering::Relaxed);
    }

    on_cleanup(move || {
        let id = handle.load(std::sync::atomic::Ordering::Relaxed);
        if id != 0
            && let Some(win) = web_sys::window() {
                let _ = win.cancel_animation_frame(id);
            }
    });
}
