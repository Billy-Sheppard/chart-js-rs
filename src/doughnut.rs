use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{types::*, ChartExt, ChartOptions};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Doughnut<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: DoughnutString,
    pub data: Dataset<Vec<SinglePointDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}
impl<A: Annotation + DeserializeOwned> ChartExt for Doughnut<A> {
    fn get_id(self) -> String {
        self.id
    }
}

#[derive(Debug, Clone, Deserialize)]
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
