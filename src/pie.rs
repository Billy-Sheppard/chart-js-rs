use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{types::*, ChartExt, ChartOptions};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Pie<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: PieString,
    pub data: Dataset<Vec<SinglePointDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation + DeserializeOwned> ChartExt for Pie<A> {
    fn get_id(self) -> String {
        self.id
    }
}

#[derive(Debug, Clone, Deserialize)]
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
