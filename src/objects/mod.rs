mod chart_objects;
mod helper_objects;
mod methods;

pub use chart_objects::*;
pub use helper_objects::*;

use js_sys::Reflect;
use serde::Deserialize;
use wasm_bindgen::JsValue;

fn rationalise_1_level<const N: usize, T: for<'a> Deserialize<'a>>(
    obj: &JsValue,
    name: &'static str,
    f: impl Fn(T),
) {
    if let Ok(a) = Reflect::get(obj, &name.into()) {
        // If the property is undefined, dont try serialize it
        if a == JsValue::UNDEFINED {
            return;
        }

        if let Ok(o) = serde_wasm_bindgen::from_value::<T>(a) {
            f(o)
        }
    }
}
fn rationalise_2_levels<const N: usize, T: for<'a> Deserialize<'a>>(
    obj: &JsValue,
    name: (&'static str, &'static str),
    f: impl Fn(JsValue, T),
) {
    if let Ok(a) = Reflect::get(obj, &name.0.into()) {
        // If the property is undefined, dont try serialize it
        if a == JsValue::UNDEFINED {
            return;
        }

        if let Ok(b) = Reflect::get(&a, &name.1.into()) {
            // If the property is undefined, dont try serialize it
            if b == JsValue::UNDEFINED {
                return;
            }

            if let Ok(o) = serde_wasm_bindgen::from_value::<T>(b) {
                f(a, o)
            }
        }
    }
}
