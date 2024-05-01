use js_sys::{Array, Function, Object, Reflect};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{render_chart, update_chart};

/// Macro to make it easier to rationalize the chart options
///
/// Pass the owning object, and the path to the FnWithArgs
macro_rules! rationalize {
    ($set:ident, $name:expr) => {
        let s = $name.split('.').collect::<Vec<&str>>();

        if let Ok(a) = Reflect::get(&$set, &s[0].into()) {
            // If the property is undefined, dont try serialize it
            if a == JsValue::UNDEFINED {
                return;
            }

            if let Ok(b) = Reflect::get(&a, &s[1].into()) {
                // If the property is undefined, dont try serialize it
                if b == JsValue::UNDEFINED {
                    return;
                }
                Reflect::set(
                    &a,
                    &s[1].into(),
                    &serde_wasm_bindgen::from_value::<FnWithArgs>(b)
                        .unwrap()
                        .build(),
                )
                .unwrap();
            }
        }
    };
}

#[wasm_bindgen]
pub struct Chart(pub(crate) JsValue, pub(crate) String);

/// Walks the JsValue object to get the value of a nested property
/// using the JS dot notation
fn get_path(j: &JsValue, item: &str) -> Option<JsValue> {
    let mut path = item.split('.');
    let item = &path.next().unwrap().to_string().into();
    let k = Reflect::get(j, item);

    if k.is_err() {
        return None;
    }

    let k = k.unwrap();
    if path.clone().count() > 0 {
        return get_path(&k, path.collect::<Vec<&str>>().join(".").as_str());
    }

    Some(k)
}

/// Get values of an object as an array at the given path.
/// See get_path()
fn object_values_at(j: &JsValue, item: &str) -> Option<Array> {
    let o = get_path(j, item);
    o.and_then(|o| {
        if o == JsValue::UNDEFINED {
            None
        } else {
            Some(Object::values(&o.dyn_into().unwrap()))
        }
    })
}

impl Chart {
    pub fn new(v: JsValue, id: String) -> Option<Self> {
        v.is_object().then_some(Self(v, id))
    }
    pub fn render(self) {
        self.rationalise_js();
        render_chart(self.0, &self.1, false);
    }
    pub fn render_mutate(self) {
        self.rationalise_js();
        render_chart(self.0, &self.1, true);
    }
    pub fn update(self, animate: bool) -> bool {
        update_chart(self.0, &self.1, animate)
    }

    /// Converts serialized FnWithArgs to JS Function's
    /// For new chart options, this will need to be updated
    pub fn rationalise_js(&self) {
        // Handle data.datasets
        Array::from(&get_path(&self.0, "data.datasets").unwrap())
            .iter()
            .for_each(|dataset| {
                rationalize!(dataset, "segment.borderDash");
                rationalize!(dataset, "segment.borderColor");
            });

        // Handle data.options.scales
        if let Some(scales) = object_values_at(&self.0, "options.scales") {
            scales.iter().for_each(|scale| {
                rationalize!(scale, "ticks.callback");
            });
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FnWithArgs {
    pub args: Vec<String>,
    pub body: String,
}

impl FnWithArgs {
    pub fn is_empty(&self) -> bool {
        self.args.is_empty() && self.body.is_empty()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn arg(&mut self, name: &str) -> &mut Self {
        self.args.push(name.to_string());
        self
    }

    pub fn body(&mut self, body: &str) -> Self {
        self.body = body.to_string();
        self.to_owned()
    }

    pub fn build(&self) -> Function {
        Function::new_with_args(&self.args.join(", "), &format!("return {}", self.body))
    }
}
