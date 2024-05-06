use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/js/exports.js")]
extern "C" {
    pub fn get_chart(id: &str) -> JsValue;

    pub fn render_chart(v: JsValue, id: &str, mutate: bool);

    pub fn update_chart(updated: JsValue, id: &str, animate: bool) -> bool;
}
