use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{types::*, ChartExt, ChartOptions};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Bar<A: Annotation> {
    #[serde(rename = "type")]
    pub r#type: BarString,
    pub data: Dataset<Vec<XYDataset>>,
    pub options: ChartOptions<A>,
    pub id: String,
}

impl<A: Annotation + DeserializeOwned> ChartExt for Bar<A> {
    fn get_id(self) -> String {
        self.id
    }
}

#[derive(Debug, Clone, Deserialize)]
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
