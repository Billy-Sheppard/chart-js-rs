use serde::Serialize;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize)]
pub struct Bar {
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: Dataset<Vec<XYDataset>>,
    pub options: ChartOptions,
    pub id: String
}

impl Bar {
    pub fn to_chart(self) -> Chart {
        Chart(JsValue::from_serde(&self).unwrap_throw())
    }
}
