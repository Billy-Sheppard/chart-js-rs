use serde::Serialize;
use wasm_bindgen::JsValue;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Scatter<A: Annotation> {
    #[serde(rename = "type", default = "_scatter_string")]
    pub r#type: String,
    pub data: Dataset<Vec<XYDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Scatter<A> {
    pub fn to_chart(self) -> Chart {
        Chart(JsValue::from_serde(&self).unwrap())
    }
}

fn _scatter_string() -> String {
    "scatter".into()
}