use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::render_chart;

#[wasm_bindgen]
pub struct Chart(pub(crate) JsValue);

impl Chart {
    pub fn new(v: JsValue) -> Option<Self> {
        v.is_object().then(|| Self(v))
    }
    pub fn render(self, id: &str) {
        render_chart(self.0, id, false);
    }
    pub fn render_mutate(self, id: &str) {
        render_chart(self.0, id, true);
    }
}
