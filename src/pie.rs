use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Pie<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: PieString,
    pub data: Dataset<Vec<SinglePointDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Pie<A> {
    pub fn to_chart(self) -> Chart {
        Chart(
            <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self).unwrap(),
            self.id,
        )
    }
}

#[derive(Debug, Clone)]
pub struct PieString(String);
impl Serialize for PieString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("pie")
    }
}
impl Default for PieString {
    fn default() -> Self {
        Self("pie".into())
    }
}
