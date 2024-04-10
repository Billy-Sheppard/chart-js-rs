use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};

use crate::{types::*, ChartExt, traits::*};

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

#[derive(Debug, Default, Clone)]

pub struct ScatterString;
impl<'de> Deserialize<'de> for ScatterString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.to_lowercase().as_str() {
            "scatter" => Ok(ScatterString),
            other => Err(de::Error::custom(format!(
                "`{other}` is not a valid ScatterString."
            ))),
        }
    }
}
impl Serialize for ScatterString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("scatter")
    }
}
