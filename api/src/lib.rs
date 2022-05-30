pub mod line;
pub mod types;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub trait Chart {
    fn to_js(id: String, obj: Self);
}

#[wasm_bindgen(
    inline_js = "export function chart(id, obj) { const ctx = document.getElementById(id); const config = new Chart(ctx, obj); }"
)]
extern "C" {
    fn chart(id: String, obj: JsValue) -> u32;
}
