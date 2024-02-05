use js_sys::{Array, Function, Reflect};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{render_chart, update_chart};

#[wasm_bindgen]
pub struct Chart(pub(crate) JsValue, pub(crate) String);

/// Walks the JsValue object to get the value of a nested property
/// using the JS dot notation
fn get_path(j: &JsValue, item: &str) -> Option<JsValue> {
    let mut path = item.split(".");
    let item = &path.next().unwrap().to_string().into();
    let k = Reflect::get(&j, item);

    if k.is_err() {
        return None;
    }

    let k = k.unwrap();
    if path.clone().count() > 0 {
        return get_path(&k, path.collect::<Vec<&str>>().join(".").as_str());
    }

    Some(k)
}

impl Chart {
    pub fn new(v: JsValue, id: String) -> Option<Self> {
        v.is_object().then_some(Self(v, id))
    }
    pub fn render(self) {
        self.correct();
        render_chart(self.0, &self.1, false);
    }
    pub fn render_mutate(self) {
        self.correct();
        render_chart(self.0, &self.1, true);
    }
    pub fn update(self, animate: bool) -> bool {
        update_chart(self.0, &self.1, animate)
    }

    /// Converts the string-serialized segment functions to a JavaScript function
    /// then updates the chart options in the Js representation opf the chart
    pub fn correct(&self) {
        Array::from(&get_path(&self.0, "data.datasets").unwrap())
            .iter()
            .for_each(|dataset| {
                let segment = Reflect::get(&dataset, &"segment".into());
                if !segment.is_err() {
                    let segment = segment.unwrap();

                    let dash = Reflect::get(&segment, &"borderDash".into());
                    if dash.is_ok() {
                        Reflect::set(
                            &segment,
                            &"borderDash".into(),
                            &serde_wasm_bindgen::from_value::<FnWithArgs>(dash.unwrap())
                                .unwrap()
                                .build(),
                        )
                        .unwrap();
                    }

                    let color = Reflect::get(&segment, &"borderColor".into());
                    if color.is_ok() {
                        Reflect::set(
                            &segment,
                            &"borderColor".into(),
                            &serde_wasm_bindgen::from_value::<FnWithArgs>(color.unwrap())
                                .unwrap()
                                .build(),
                        )
                        .unwrap();
                    }
                }
            });
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

pub fn some_false() -> Option<bool> {
    Some(false)
}
