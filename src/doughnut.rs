use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Doughnut<A: Annotation> {
    #[serde(rename = "type", default = "_doughnut_string")]
    pub r#type: String,
    pub data: Dataset<Vec<SinglePointDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Doughnut<A> {
    pub fn to_chart(self) -> Chart {
        Chart(<::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self).unwrap())
    }
}

fn _doughnut_string() -> String {
    "doughnut".into()
}
