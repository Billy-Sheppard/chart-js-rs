use {
    crate::{traits::*, utils::FnWithArgs},
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, fmt::Debug, fmt::Display},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct DatasetData(pub(crate) serde_json::Value);
impl DatasetData {
    fn is_empty(&self) -> bool {
        serde_json::to_value(self)
            .unwrap()
            .as_array()
            .unwrap()
            .is_empty()
    }
}
impl PartialOrd for DatasetData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DatasetData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.to_string().cmp(&other.0.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct NoDatasets {}
impl DatasetTrait for NoDatasets {}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct NoAnnotations {}
impl Annotation for NoAnnotations {}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dataset<D: DatasetTrait> {
    pub datasets: D,
    pub labels: Option<Vec<NumberOrDateString>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum StringOrInt {
    String(String),
    Int(isize),
    Vec(Vec<()>),
}
impl Display for StringOrInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOrInt::String(s) => write!(f, "{s}"),
            StringOrInt::Int(i) => write!(f, "{i}"),
            StringOrInt::Vec(_) => write!(f, ""),
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NumberOrDateString(String);
impl From<NumberString> for NumberOrDateString {
    fn from(value: NumberString) -> Self {
        value.0.into()
    }
}
impl NumberOrDateString {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl PartialOrd for NumberOrDateString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NumberOrDateString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some((s, o)) = self
            .0
            .parse::<rust_decimal::Decimal>()
            .ok()
            .zip(other.0.parse::<rust_decimal::Decimal>().ok())
        {
            s.cmp(&o)
        } else {
            self.0.cmp(&other.0)
        }
    }
}
impl<T: Display> From<T> for NumberOrDateString {
    fn from(s: T) -> Self {
        Self(s.to_string())
    }
}
#[allow(unknown_lints, clippy::to_string_trait_impl)]
impl ToString for NumberOrDateString {
    fn to_string(&self) -> String {
        self.0.to_string()
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
impl<'de> Deserialize<'de> for NumberOrDateString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        StringOrInt::deserialize(deserializer).map(|soi| Self(soi.to_string()))
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoolString(String);
impl BoolString {
    pub fn true_() -> Option<BoolString> {
        BoolString("true".into()).into()
    }
    pub fn false_() -> Option<BoolString> {
        BoolString("false".into()).into()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<T: Display> From<T> for BoolString {
    fn from(s: T) -> Self {
        Self(s.to_string())
    }
}
impl Serialize for BoolString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bool_: Result<bool, _> = self.0.parse();
        let any: Result<String, _> = self.0.parse();
        match (bool_, any) {
            (Ok(bool_), _) => serializer.serialize_bool(bool_),
            (_, Ok(any)) => serializer.serialize_str(&any),
            _ => unreachable!(),
        }
    }
}
impl<'de> Deserialize<'de> for BoolString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        StringOrInt::deserialize(deserializer).map(|soi| Self(soi.to_string()))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NumberString(String);
impl From<NumberOrDateString> for NumberString {
    fn from(value: NumberOrDateString) -> Self {
        value.0.into()
    }
}
impl NumberString {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl PartialOrd for NumberString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NumberString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some((s, o)) = self
            .0
            .parse::<rust_decimal::Decimal>()
            .ok()
            .zip(other.0.parse::<rust_decimal::Decimal>().ok())
        {
            s.cmp(&o)
        } else {
            self.0.cmp(&other.0)
        }
    }
}
impl<T: Display> From<T> for NumberString {
    fn from(s: T) -> Self {
        Self(s.to_string())
    }
}
#[allow(unknown_lints, clippy::to_string_trait_impl)]
impl ToString for NumberString {
    fn to_string(&self) -> String {
        self.0.to_string()
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
impl<'de> Deserialize<'de> for NumberString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        StringOrInt::deserialize(deserializer).map(|soi| Self(soi.to_string()))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SinglePointDataset {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub backgroundColor: Vec<String>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub base: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderJoinStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderSkipped: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderWidth: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub clip: NumberString,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub data: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datalabels: Option<DataLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hoverBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hoverBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverBorderRadius: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverBorderWidth: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub indexAxis: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub inflateAmount: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub maxBarThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub minBarLength: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub order: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointBorderWidth: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointHoverBackgroundColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointHoverBorderWidth: NumberString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub pointHoverRadius: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointStyle: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipNull: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stack: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepped: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub xAxisID: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub yAxisID: String,
}
impl DatasetTrait for Vec<SinglePointDataset> {}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct XYDataset {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub base: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderJoinStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderSkipped: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderWidth: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub category_label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub clip: NumberString,
    #[serde(skip_serializing_if = "DatasetData::is_empty", default)]
    pub data: DatasetData,
    /// Use Default::default() if this isn't required
    pub datalabels: DataLabels,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub fill: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hitRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hoverBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hoverBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverBorderRadius: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverBorderWidth: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub indexAxis: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub inflateAmount: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub maxBarThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub minBarLength: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub order: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointBorderWidth: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointHitRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointHoverBackgroundColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointHoverBorderWidth: NumberString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub pointHoverRadius: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointStyle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipNull: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spanGaps: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stack: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepped: Option<BoolString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub tension: NumberString,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub r#type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub xAxisID: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub yAxisID: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub z: NumberString,
}
impl DatasetTrait for Vec<XYDataset> {}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub(crate) struct XYPoint {
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub x: NumberOrDateString,

    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub y: NumberString,

    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
}
impl PartialOrd for XYPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for XYPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x)
    }
}

pub type MinMaxPoint = [NumberOrDateString; 2];

impl From<(NumberOrDateString, NumberString, Option<String>)> for XYPoint {
    fn from((x, y, d): (NumberOrDateString, NumberString, Option<String>)) -> Self {
        XYPoint {
            x,
            y,
            description: d.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct ChartOptions<A: Annotation> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<ChartElements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction: Option<ChartInteraction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<ChartLegend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainAspectRatio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<ChartPlugins<A>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scales: Option<HashMap<String, ChartScale>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spanGaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltips: Option<ChartTooltips>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Animation {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub duration: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct ChartPlugins<A: Annotation> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotations<A>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocolors: Option<AutoColors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<PluginLegend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipPlugins>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PluginLegend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LegendLabel>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct Annotations<A: Annotation> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, A>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AutoColors {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TooltipPlugins {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub bodyAlign: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub bodyColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<TooltipCallbacks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayColors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub titleAlign: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub titleColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub titleMarginBottom: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TooltipCallbacks {
    #[serde(skip_serializing_if = "FnWithArgs::is_empty")]
    pub label: FnWithArgs,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartScale {
    #[serde(skip_serializing_if = "FnWithArgs::is_empty")]
    pub afterBuildTicks: FnWithArgs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignToPixels: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColour: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barPercentage: NumberString,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty")]
    pub beforeFit: FnWithArgs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beginAtZero: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<ScaleBorder>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub bounds: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub grace: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<Grid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped: Option<bool>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub max: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub min: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacked: Option<bool>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub suggestedMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub suggestedMin: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<ScaleTicks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<ScaleTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub weight: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleBorder {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub dash: Vec<NumberString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub dashOffset: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub width: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawOnChartArea: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default, skip_deserializing)]
    pub tickColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineAnnotation {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderWidth: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub drawTime: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub xMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub xMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub yMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub yMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub yScaleID: NumberString,
}
impl Annotation for LineAnnotation {}
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoxAnnotation {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderWidth: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub drawTime: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub xMax: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub xMin: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub yMax: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub yMin: String,
}
impl Annotation for BoxAnnotation {}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayFormats: Option<DisplayFormats>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub unit: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayFormats {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub day: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hour: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub minute: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub month: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub quarter: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub week: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub year: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleTicks {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub align: String,
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing
    )]
    pub callback: FnWithArgs,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub count: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub maxTicksLimit: NumberString,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub padding: Option<Padding>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub precision: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub stepSize: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Title {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartInteraction {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub axis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersect: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartTooltips {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartLegend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LegendLabel>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LegendLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxHeight: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxWidth: Option<usize>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty")]
    pub filter: FnWithArgs,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointStyleWidth: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useBorderRadius: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usePointStyle: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartElements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<BarElementConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<LineElementConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<PointElementConfiguration>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BarElementConfiguration {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderWidth: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverBorderWidth: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineElementConfiguration {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderWidth: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub cubicInterpolationMode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointElementConfiguration {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderWidth: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hitRadius: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverBorderWidth: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverRadius: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub radius: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataLabels {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub align: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub anchor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(default = "BoolString::false_")]
    pub display: Option<BoolString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub drawTime: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    pub formatter: FnWithArgs,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub offset: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Padding {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub bottom: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub left: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub right: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub top: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Font {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub family: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub lineHeight: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub size: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub style: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub weight: NumberString,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing
    )]
    pub borderColor: FnWithArgs,
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing
    )]
    pub borderDash: FnWithArgs,
}
