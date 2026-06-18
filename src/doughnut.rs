use serde::{
    de::{self},
    Deserialize, Serialize,
};

use crate::{objects::*, ChartExt};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Doughnut {
    #[serde(rename = "type")]
    r#type: DoughnutString,
    data: Dataset<Vec<SinglePointDataset>>,
    options: ChartOptions,
    id: String,
}
impl ChartExt for Doughnut {
    type DS = Dataset<Vec<SinglePointDataset>>;

    fn get_id(&self) -> &str {
        &self.id
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
