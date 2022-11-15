use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Scatter<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: ScatterString,
    pub data: Dataset<Vec<XYDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Scatter<A> {
    pub fn to_chart(self) -> Chart {
        Chart(<::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self).unwrap(), self.id)
    }
}

#[derive(Debug, Clone)]
pub struct ScatterString(String);
impl Serialize for ScatterString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("scatter")
    }
}
impl Default for ScatterString {
    fn default() -> Self {
        Self("scatter".into())
    }
}
