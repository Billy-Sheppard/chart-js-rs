use serde::{
    de::{self},
    Deserialize, Serialize,
};

use crate::{objects::*, ChartExt};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Scatter {
    #[serde(rename = "type")]
    r#type: ScatterString,
    data: Dataset<Vec<XYDataset>>,
    options: ChartOptions,
    id: String,
}

impl ChartExt for Scatter {
    type DS = Dataset<Vec<XYDataset>>;

    fn get_id(self) -> String {
        self.id
    }
    fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    fn get_data(&mut self) -> &mut Self::DS {
        &mut self.data
    }

    fn get_options(&mut self) -> &mut ChartOptions {
        &mut self.options
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
