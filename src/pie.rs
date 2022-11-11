use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Pie<A: Annotation> {
    #[serde(rename = "type", default = "_pie_string")]
    pub r#type: String,
    pub data: Dataset<Vec<SinglePointDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Pie<A> {
    pub fn to_chart(self) -> Chart {
        Chart(<::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self).unwrap())
    }
}

fn _pie_string() -> String {
    "pie".into()
}
