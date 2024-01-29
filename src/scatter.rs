use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{types::*, ChartExt, ChartOptions};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Scatter<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: ScatterString,
    pub data: Dataset<Vec<XYDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation + DeserializeOwned> ChartExt for Scatter<A> {
    fn get_id(self) -> String {
        self.id
    }
}

#[derive(Debug, Deserialize, Clone)]
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
