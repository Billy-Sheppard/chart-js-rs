#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]

pub mod bar;
pub mod doughnut;
pub mod functions;
pub mod pie;
pub mod scatter;
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

    console.debug(obj);

    const ctx = document.getElementById(id);
    var chart = new Chart(ctx, obj);
}"#)]
extern "C" {
    fn render_chart(v: JsValue, id: &str, mutate: bool);
}
