#![allow(non_snake_case)]

pub mod functions;
pub mod scatter;
pub mod bar;
pub mod types;

#[doc(hidden)]
pub mod utils;

pub use types::*;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(inline_js = r#"export function render_chart(v, id, mutate) {
    console.debug(v);

    let obj;
    if (mutate) {
        obj = mutate_chart_object(v)
    }
    else {
        obj = v
    };

    console.log(obj);

    const ctx = document.getElementById(id);
    const config = new Chart(ctx, obj);
}"#)]
extern "C" {
    fn render_chart(v: JsValue, id: &str, mutate: bool);
}
