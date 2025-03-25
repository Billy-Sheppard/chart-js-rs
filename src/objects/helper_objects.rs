use {
    crate::{traits::*, utils::FnWithArgs},
    serde::{
        de::{self, DeserializeOwned},
        Deserialize, Serialize,
    },
    std::fmt::{Debug, Display},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct DatasetData(pub serde_json::Value);
impl DatasetData {
    pub fn is_empty(&self) -> bool {
        serde_json::to_value(self)
            .unwrap()
            .as_array()
            .unwrap()
            .is_empty()
    }

    pub fn from_single_point_array(iter: impl Iterator<Item = [NumberOrDateString; 1]>) -> Self {
        DatasetData(serde_json::to_value(iter.collect::<Vec<_>>()).unwrap())
    }

    pub fn from_minmax_array(iter: impl Iterator<Item = [NumberOrDateString; 2]>) -> Self {
        DatasetData(serde_json::to_value(iter.collect::<Vec<_>>()).unwrap())
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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct NoDatasets {}
impl DatasetTrait for NoDatasets {
    fn labels(self) -> Vec<NumberOrDateString> {
        Vec::new()
    }
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct NoAnnotations {}
impl Annotation for NoAnnotations {}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dataset<D: DatasetTrait> {
    datasets: D,
    labels: Option<Vec<NumberOrDateString>>,
}
impl<D: DatasetTrait> Dataset<D> {
    pub fn new() -> Self {
        Self {
            datasets: D::default(),
            labels: None,
        }
    }
    pub fn get_datasets(&mut self) -> &mut D {
        &mut self.datasets
    }
    pub fn datasets(mut self, datasets: impl Into<D>) -> Self {
        self.datasets = datasets.into();
        let labels = self.datasets.clone();
        self.labels(labels.labels())
    }
    pub fn get_labels(&mut self) -> &mut Option<Vec<NumberOrDateString>> {
        &mut self.labels
    }
    pub fn labels<T: Into<NumberOrDateString>>(
        mut self,
        labels: impl IntoIterator<Item = T>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any {
    String(String),
    Int(isize),
    Bool(bool),
    Vec(Vec<()>),
}
impl From<bool> for Any {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl From<String> for Any {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl Any {
    pub fn is_empty(&self) -> bool {
        match self {
            Any::String(s) => s.is_empty(),
            Any::Int(_i) => false,
            Any::Bool(_b) => false,
            Any::Vec(v) => v.is_empty(),
        }
    }
}
impl Display for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Any::String(s) => write!(f, "{s}"),
            Any::Bool(b) => write!(f, "{b}"),
            Any::Int(i) => write!(f, "{i}"),
            Any::Vec(_) => write!(f, ""),
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
        Any::deserialize(deserializer).map(|soi| Self(soi.to_string()))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoolString(String);
impl BoolString {
    pub fn opt_true() -> Option<BoolString> {
        BoolString("true".into()).into()
    }
    pub fn opt_false() -> Option<BoolString> {
        BoolString("false".into()).into()
    }
    pub fn _true() -> BoolString {
        BoolString("true".into())
    }
    pub fn _false() -> BoolString {
        BoolString("false".into())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl ChartJsRsObject for BoolString {
    fn is_empty(&self) -> bool {
        self.is_empty()
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
        Any::deserialize(deserializer).map(|soi| Self(soi.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FnWithArgsOrT<const N: usize, T> {
    T(T),
    FnWithArgs(FnWithArgs<N>),
}
#[allow(private_bounds)]
impl<const N: usize, T: ChartJsRsObject> FnWithArgsOrT<N, T> {
    pub fn is_empty(&self) -> bool {
        match self {
            FnWithArgsOrT::T(a) => a.is_empty(),
            FnWithArgsOrT::FnWithArgs(fnwa) => fnwa.is_empty(),
        }
    }
}
impl<const N: usize, T: Default> Default for FnWithArgsOrT<N, T> {
    fn default() -> Self {
        FnWithArgsOrT::T(T::default())
    }
}
impl<const N: usize, T: Into<String>> From<T> for FnWithArgsOrT<N, String> {
    fn from(s: T) -> Self {
        Self::T(s.into())
    }
}
impl<const N: usize, T: Into<NumberString>> From<T> for FnWithArgsOrT<N, NumberString> {
    fn from(ns: T) -> Self {
        Self::T(ns.into())
    }
}
impl<const N: usize, T: Into<BoolString>> From<T> for FnWithArgsOrT<N, BoolString> {
    fn from(bs: T) -> Self {
        Self::T(bs.into())
    }
}
impl<const N: usize, T> From<FnWithArgs<N>> for FnWithArgsOrT<N, T> {
    fn from(value: FnWithArgs<N>) -> Self {
        Self::FnWithArgs(value)
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
impl ChartJsRsObject for NumberString {
    fn is_empty(&self) -> bool {
        self.is_empty()
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
#[allow(clippy::to_string_trait_impl)]
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
        Any::deserialize(deserializer).map(|soi| Self(soi.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(untagged)]
pub enum NumberStringOrT<T: Serialize + DeserializeOwned> {
    T(T),
    NumberString(NumberString),
}
impl<'de, T: Serialize + DeserializeOwned> Deserialize<'de> for NumberStringOrT<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        // thanks serde :|
        let value = serde::__private::de::Content::deserialize(deserializer)?;
        let deserializer = serde::__private::de::ContentRefDeserializer::<D::Error>::new(&value);

        match NumberString::deserialize(deserializer) {
            Ok(ns) => Ok(Self::NumberString(ns)),
            Err(_) => T::deserialize(deserializer).map(Self::T),
        }
    }
}
impl<T: Serialize + DeserializeOwned> NumberStringOrT<T> {
    pub fn is_empty(&self) -> bool {
        match self {
            NumberStringOrT::T(_t) => false,
            NumberStringOrT::NumberString(ns) => ns.is_empty(),
        }
    }
}

impl<T: Serialize + ChartJsRsObject, U: Serialize + DeserializeOwned> From<T>
    for NumberStringOrT<U>
{
    fn from(value: T) -> Self {
        serde_json::from_value(serde_json::to_value(value).unwrap()).unwrap()
    }
}
