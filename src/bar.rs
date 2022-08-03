use serde::Serialize;
use wasm_bindgen::JsValue;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Bar<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: Dataset<Vec<XYDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Bar<A> {
    pub fn to_chart(self) -> Chart {
        Chart(JsValue::from_serde(&self).unwrap())
    }
}
