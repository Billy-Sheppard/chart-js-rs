use serde::{Deserialize, Serialize};
use std::option::Option;
use std::{collections::HashMap, fmt::Display};

pub trait DatasetTrait: Serialize {}
pub trait Annotation: Serialize {}

#[derive(Debug, Serialize, Default)]
pub struct NoDatasets {}
impl DatasetTrait for NoDatasets {}
#[derive(Debug, Serialize, Default)]
pub struct NoAnnotations {}
impl Annotation for NoAnnotations {}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Dataset<D: DatasetTrait> {
    pub datasets: D,
    pub labels: Option<Vec<NumberOrDateString>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(transparent)]
pub struct NumberOrDateString(String);
impl NumberOrDateString {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
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
impl NumberString {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
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
pub struct SinglePointDataset {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<NumberString>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub borderColor: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub barThickness: NumberString,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub backgroundColor: Vec<String>,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub order: NumberString,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub pointRadius: NumberString,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub pointHoverRadius: NumberOrDateString,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub pointStyle: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub pointBackgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub pointHoverBackgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub pointBorderColor: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub pointBorderWidth: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub pointHoverBorderWidth: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderWidth: NumberString,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub borderDash: Vec<NumberString>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub borderJoinStyle: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub hoverBackgroundColor: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepped: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub yAxisID: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub xAxisID: String,
}
impl DatasetTrait for Vec<SinglePointDataset> {}

#[derive(Debug, Clone, Serialize, Default)]
pub struct XYDataset {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<XYPoint>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub borderColor: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub barThickness: NumberString,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub backgroundColor: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub order: NumberString,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub pointRadius: NumberString,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub pointHoverRadius: NumberOrDateString,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub pointStyle: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub pointBackgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub pointHoverBackgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub pointBorderColor: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub pointBorderWidth: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub pointHoverBorderWidth: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderWidth: NumberString,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub borderDash: Vec<NumberString>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub borderJoinStyle: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub hoverBackgroundColor: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepped: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub yAxisID: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub xAxisID: String,
}
impl DatasetTrait for Vec<XYDataset> {}

#[derive(Debug, Clone, Serialize, Default)]
pub struct XYPoint {
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub x: NumberOrDateString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub y: NumberString,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartOptions<A: Annotation> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<ChartPlugins<A>>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spanGaps: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<ChartElements>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Animation {
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub duration: NumberString,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartPlugins<A: Annotation> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocolors: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipPlugins>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotations<A>>,

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
pub struct Annotations<A: Annotation> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, A>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TooltipPlugins {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub bodyColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub bodyAlign: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayColors: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub backgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub titleColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub titleAlign: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub titleMarginBottom: NumberString,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartScale {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<ScaleTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<Grid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<ScaleTicks>,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub grace: NumberOrDateString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub bounds: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub position: String,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub min: NumberOrDateString,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub max: NumberOrDateString,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub suggestedMin: NumberOrDateString,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub suggestedMax: NumberOrDateString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub beginAtZero: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacked: Option<bool>,
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
    #[serde(skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub drawTime: String,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub xMin: NumberOrDateString,

    #[serde(skip_serializing_if = "NumberOrDateString::is_empty")]
    pub xMax: NumberOrDateString,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub borderColor: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub borderDash: Vec<NumberString>,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderWidth: NumberString,
}
impl Annotation for LineAnnotation {}
#[derive(Debug, Clone, Serialize, Default)]
pub struct BoxAnnotation {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub r#type: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub drawTime: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub xMin: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub xMax: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub yMin: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub yMax: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub borderColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub backgroundColor: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub borderDash: Vec<NumberString>,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderWidth: NumberString,
}
impl Annotation for BoxAnnotation {}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ScaleTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayFormats: Option<DisplayFormats>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct DisplayFormats {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub year: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub quarter: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub month: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub week: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub day: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub hour: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub minute: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ScaleTicks {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub align: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub maxTicksLimit: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub stepSize: NumberString,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Title {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartInteraction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersect: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub mode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub axis: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartTooltips {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub position: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartLegend {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub position: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LegendLabel>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct LegendLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usePointStyle: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartElements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<BarElementConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<LineElementConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<PointElementConfiguration>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct BarElementConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderRadius: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderWidth: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub hoverBorderWidth: NumberString,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct LineElementConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderWidth: NumberString,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub cubicInterpolationMode: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct PointElementConfiguration {
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub radius: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub hitRadius: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub hoverRadius: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderWidth: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub hoverBorderWidth: NumberString,
}
