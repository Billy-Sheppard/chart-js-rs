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

use gloo_utils::format::JsValueSerdeExt;
use serde::{de::DeserializeOwned, Serialize};
pub use types::*;

use utils::Chart;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub trait ChartExt: DeserializeOwned + Serialize {
    fn get_id(self) -> String;

    fn into_chart(self) -> Chart {
        Chart(
            <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self)
                .expect("Unable to serialize chart."),
            self.get_id(),
        )
    }

    fn get_chart_from_id(id: &str) -> Option<Self> {
        let chart = get_chart(id);
        serde_wasm_bindgen::from_value(chart)
            .map_err(|e| {
                gloo_console::error!("{}", e.to_string());
                e
            })
            .ok()
    }
}

#[wasm_bindgen(inline_js = r#"export function render_chart(v, id, mutate) {
    console.debug('Before mutate:', v);

    let obj;
    if (mutate) {
        obj = mutate_chart_object(v)
    }
    else {
        obj = v
    };

    console.debug('After mutate:', obj);

    const ctx = document.getElementById(id);
    let chart = new Chart(ctx, obj);
}"#)]
extern "C" {
    fn render_chart(v: JsValue, id: &str, mutate: bool);
}

#[wasm_bindgen(inline_js = r#"export function get_chart(id) {
    return Chart.getChart(document.getElementById(id)).config._config
}"#)]
extern "C" {
    fn get_chart(id: &str) -> JsValue;
}

#[wasm_bindgen(inline_js = r#"export function update_chart(updated, id, animate) {
    try {
        let chart = Chart.getChart(document.getElementById(id));
        chart.config._config.type = updated.type;
        chart.config._config.data = updated.data;
        chart.config._config.options = updated.options;

        console.debug('Updated chart:', chart);

        if (animate) {
            chart.update();
        } else {
            chart.update('none');
        }

        true
    }
    catch {
        false
    }
}"#)]
extern "C" {
    fn update_chart(updated: JsValue, id: &str, animate: bool) -> bool;
}
