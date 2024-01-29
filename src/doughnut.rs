use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;

use crate::{types::*, utils::*, ChartOptions};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Doughnut<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: DoughnutString,
    pub data: Dataset<Vec<SinglePointDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation> Doughnut<A> {
    pub fn to_chart(self) -> Chart {
        Chart(
            <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self)
                .expect("Unable to serialize chart."),
            self.id,
        )
    }
}

#[derive(Debug, Clone)]
pub struct DoughnutString(String);
impl Serialize for DoughnutString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("doughnut")
    }
}
impl Default for DoughnutString {
    fn default() -> Self {
        Self("doughnut".into())
    }
}
