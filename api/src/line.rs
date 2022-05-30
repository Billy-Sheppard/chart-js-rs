use crate::{chart, types::*, Chart};

use serde::Serialize;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Serialize)]
pub struct LineData {
    #[serde(flatten)]
    pub datasets: Vec<LineDataset>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LineDataset {
    pub a: Option<ControllerDatasetOptionsProperties>,
    pub b: Option<SegmentProperties>,
    pub c: Option<LineOptionsProperties>,
    pub d: Option<LineDataPoint>,
}
#[derive(Debug, Clone, Serialize)]
pub struct LineDataPoint {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LineChart {
    #[serde(rename = "type", default = "line")]
    pub type_: String,
    pub options: RegistryProperties,
    pub data: LineData,
}

impl Chart for LineChart {
    fn to_js(id: String, obj: LineChart) {
        chart(id, JsValue::from_serde(&obj).unwrap());
    }
}
