use serde::{
    de::{self},
    Deserialize, Serialize,
};

use crate::{objects::*, ChartExt, DatasetTrait};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(bound = "D: DatasetTrait")]
pub struct Bar<D: DatasetTrait> {
    #[serde(rename = "type")]
    r#type: BarString,
    data: Dataset<D>,
    options: ChartOptions,
    id: String,
}

impl<D: DatasetTrait> ChartExt for Bar<D> {
    type DS = Dataset<D>;

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
