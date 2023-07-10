use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::render_chart;

#[wasm_bindgen]
pub struct Chart(pub(crate) JsValue, pub(crate) String);

impl Chart {
    pub fn new(v: JsValue, id: String) -> Option<Self> {
        v.is_object().then_some(Self(v, id))
    }
    pub fn render(self) {
        render_chart(self.0, &self.1, false);
    }
    pub fn render_mutate(self) {
        render_chart(self.0, &self.1, true);
    }
}

pub fn some_false() -> Option<bool> {
    Some(false)
}
