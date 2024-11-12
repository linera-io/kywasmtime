use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type GlobalScope;
    pub type Performance;

    #[wasm_bindgen(method, structural, getter, js_name = "performance")]
    pub fn performance(this: &GlobalScope) -> Performance;

    #[wasm_bindgen(method, structural, js_name = "now")]
    pub fn now(this: &Performance) -> f64;

    #[wasm_bindgen(method, structural, getter, js_name = "timeOrigin")]
    pub fn time_origin(this: &Performance) -> f64;

    #[wasm_bindgen(catch, method, structural, js_name = "setTimeout")]
    pub fn set_timeout_with_callback_and_timeout_and_arguments_0(
        this: &GlobalScope,
        handler: &js_sys::Function,
        timeout: i32,
    ) -> Result<i32, JsValue>;

    #[wasm_bindgen(catch, method, structural, js_name = "clearTimeout")]
    pub fn clear_timeout_with_handle(this: &GlobalScope, handle: i32) -> Result<(), JsValue>;
}
