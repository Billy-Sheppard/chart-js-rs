use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Serialize,
};

use crate::{types::*, ChartExt};

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

#[derive(Debug, Default, Clone)]
pub struct BarString;
impl<'de> Deserialize<'de> for BarString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.to_lowercase().as_str() {
            "bar" => Ok(BarString),
            other => Err(de::Error::custom(format!(
                "`{other}` is not a valid BarString."
            ))),
        }
    }
}
impl Serialize for BarString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("bar")
    }
}
