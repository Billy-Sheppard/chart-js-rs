use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};

use crate::{traits::*, types::*, ChartExt};

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

#[derive(Debug, Default, Clone)]
pub struct DoughnutString;
impl<'de> Deserialize<'de> for DoughnutString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.to_lowercase().as_str() {
            "doughnut" => Ok(DoughnutString),
            other => Err(de::Error::custom(format!(
                "`{other}` is not a valid DoughnutString."
            ))),
        }
    }
}
impl Serialize for DoughnutString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("doughnut")
    }
}
