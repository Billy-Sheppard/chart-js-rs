use serde::{Deserialize, Serialize};
use std::option::Option;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Serialize, Default)]
pub struct Dataset<T> {
    pub datasets: T,
}
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(transparent)]
pub struct NumberOrDateString(String);
impl<T: Display> From<T> for NumberOrDateString {
    fn from(s: T) -> Self {
        Self(s.to_string())
    }
}
impl Serialize for NumberOrDateString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let fnum: Result<f64, _> = self.0.parse();
        let inum: Result<i64, _> = self.0.parse();
        match (fnum, inum) {
            (Ok(_), Ok(inum)) => serializer.serialize_i64(inum),
            (Ok(fnum), _) => serializer.serialize_f64(fnum),
            _ => serializer.serialize_str(&self.0),
        }
    }
}
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(transparent)]
pub struct NumberString(String);
impl<T: Display> From<T> for NumberString {
    fn from(s: T) -> Self {
        Self(s.to_string())
    }
}
impl Serialize for NumberString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let fnum: Result<f64, _> = self.0.parse();
        let inum: Result<i64, _> = self.0.parse();
        match (fnum, inum) {
            (Ok(_), Ok(inum)) => serializer.serialize_i64(inum),
            (Ok(fnum), _) => serializer.serialize_f64(fnum),
            _ => serializer.serialize_str(&self.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct XYDataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<XYPoint>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderColor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub barThickness: Option<NumberString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backgroundColor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<NumberString>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointRadius: Option<NumberString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointHoverRadius: Option<NumberOrDateString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointStyle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderDash: Option<Vec<NumberString>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderJoinStyle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepped: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub yAxisID: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xAxisID: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct XYPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<NumberOrDateString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<NumberString>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<ChartPlugins>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scales: Option<HashMap<String, ChartScale>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction: Option<ChartInteraction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltips: Option<ChartTooltips>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainAspectRatio: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<ChartLegend>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Animation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<NumberString>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartPlugins {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocolors: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipPlugins>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotations>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<PluginLegend>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct PluginLegend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Default)]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, LineAnnotation>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TooltipPlugins {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartScale {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<ScaleTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<Grid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<ScaleTicks>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace: Option<NumberOrDateString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<NumberOrDateString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<NumberOrDateString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestedMin: Option<NumberOrDateString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestedMax: Option<NumberOrDateString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Grid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawOnChartArea: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct LineAnnotation {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xMin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xMax: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderColor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderDash: Option<Vec<NumberString>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberString>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ScaleTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayFormats: Option<DisplayFormats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct DisplayFormats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ScaleTicks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxTicksLimit: Option<NumberString>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepSize: Option<NumberString>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Title {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartInteraction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartTooltips {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartLegend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LegendLabel>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct LegendLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usePointStyle: Option<bool>,
}
