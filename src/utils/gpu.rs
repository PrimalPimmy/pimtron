use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

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
