use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Bar<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: BarString,
    pub data: Dataset<Vec<XYDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Bar<A> {
    pub fn to_chart(self) -> Chart {
        Chart(
            <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self).expect("Unable to serialize chart."),
            self.id,
        )
    }
}
#[derive(Debug, Clone)]
pub struct BarString(String);
impl Serialize for BarString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("bar")
    }
}
impl Default for BarString {
    fn default() -> Self {
        Self("bar".into())
    }
}
