use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Line<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: LineString,
    pub data: Dataset<Vec<SinglePointDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Line<A> {
    pub fn to_chart(self) -> Chart {
        Chart(
            <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self).unwrap(),
            self.id,
        )
    }
}

#[derive(Debug, Clone)]
pub struct LineString(String);
impl Serialize for LineString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("line")
    }
}
impl Default for LineString {
    fn default() -> Self {
        Self("line".into())
    }
}
