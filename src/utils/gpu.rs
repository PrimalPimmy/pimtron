//! GPU capability detection and FPS tracking utilities.

use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// Interval in milliseconds for FPS calculation updates.
const FPS_UPDATE_INTERVAL_MS: f64 = 1000.0;

/// Type alias for the animation frame callback closure.
type AnimationCallback = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

/// Checks if WebGL (1 or 2) is supported in the current browser.
pub fn check_webgl_support() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };

    let Ok(canvas) = document.create_element("canvas") else {
        return false;
    };
    let Ok(canvas_el) = canvas.dyn_into::<HtmlCanvasElement>() else {
        return false;
    };

    // Try webgl2 first, then webgl
    if let Ok(Some(_)) = canvas_el.get_context("webgl2") {
        return true;
    }
    if let Ok(Some(_)) = canvas_el.get_context("webgl") {
        return true;
    }

    false
}

/// Checks if WebGPU is supported in the current browser.
pub async fn check_webgpu_support() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let navigator = window.navigator();
    let navigator_js: &wasm_bindgen::JsValue = navigator.as_ref();

    // Check if navigator.gpu exists
    let Ok(gpu_val) = js_sys::Reflect::get(navigator_js, &"gpu".into()) else {
        return false;
    };
    if gpu_val.is_undefined() || gpu_val.is_null() {
        return false;
    }

    // Get requestAdapter function
    let Ok(request_adapter_val) = js_sys::Reflect::get(&gpu_val, &"requestAdapter".into()) else {
        return false;
    };
    let Ok(request_adapter_fn) = request_adapter_val.dyn_into::<js_sys::Function>() else {
        return false;
    };

    // Call requestAdapter()
    let Ok(promise_val) = request_adapter_fn.call0(&gpu_val) else {
        return false;
    };

    let promise = promise_val.unchecked_into::<js_sys::Promise>();

    matches!(
        wasm_bindgen_futures::JsFuture::from(promise).await,
        Ok(adapter) if !adapter.is_null() && !adapter.is_undefined()
    )
}

/// Tracks FPS using requestAnimationFrame and updates the provided signal.
///
/// The FPS count is updated every second. Cleanup is handled automatically
/// when the component is unmounted.
pub fn track_fps(set_fps: WriteSignal<i32>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(performance) = window.performance() else {
        return;
    };

    let last_time = Rc::new(RefCell::new(performance.now()));
    let frame_count = Rc::new(RefCell::new(0i32));
    // Use Arc<AtomicI32> for on_cleanup which requires Send+Sync
    let handle = Arc::new(AtomicI32::new(0));

    let f: AnimationCallback = Rc::new(RefCell::new(None));
    let g = f.clone();

    let window_clone = window.clone();
    let performance_clone = performance.clone();
    let handle_clone = handle.clone();

    let loop_fn = move || {
        let now = performance_clone.now();
        *frame_count.borrow_mut() += 1;

        let delta = now - *last_time.borrow();
        if delta >= FPS_UPDATE_INTERVAL_MS {
            set_fps.set(*frame_count.borrow());
            *frame_count.borrow_mut() = 0;
            *last_time.borrow_mut() = now;
        }

        if let Some(cb) = g.borrow().as_ref() {
            let id = window_clone
                .request_animation_frame(cb.as_ref().unchecked_ref())
                .unwrap_or(0);
            handle_clone.store(id, Ordering::Relaxed);
        }
    };

    *f.borrow_mut() = Some(Closure::wrap(Box::new(loop_fn) as Box<dyn FnMut()>));

    if let Some(cb) = f.borrow().as_ref() {
        let id = window
            .request_animation_frame(cb.as_ref().unchecked_ref())
            .unwrap_or(0);
        handle.store(id, Ordering::Relaxed);
    }

    on_cleanup(move || {
        let id = handle.load(Ordering::Relaxed);
        if id != 0
            && let Some(win) = web_sys::window()
        {
            let _ = win.cancel_animation_frame(id);
        }
    });
}
