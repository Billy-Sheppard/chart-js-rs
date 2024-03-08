use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};

use crate::{types::*, ChartExt};

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

#[derive(Debug, Default, Clone)]
pub struct PieString;
impl<'de> Deserialize<'de> for PieString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.to_lowercase().as_str() {
            "pie" => Ok(PieString),
            other => Err(de::Error::custom(format!(
                "`{other}` is not a valid PieString."
            ))),
        }
    }
}
impl Serialize for PieString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("pie")
    }
}
