use serde::Serialize;
use wasm_bindgen::JsValue;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize)]
pub struct Scatter {
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: Dataset<Vec<XYDataset>>,
    pub options: ChartOptions,
    pub id: String,
}

impl Scatter {
    pub fn to_chart(self) -> Chart {
        Chart(JsValue::from_serde(&self).unwrap())
    }
}
