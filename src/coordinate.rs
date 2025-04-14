#![allow(dead_code)]

use {
    gloo_utils::format::JsValueSerdeExt,
    js_sys::{Array, Reflect},
    std::{
        any::type_name,
        error::Error,
        fmt::{self, Display},
        str::FromStr,
    },
    wasm_bindgen::{JsCast, JsValue},
};

/// All the possible error states that result from asserting a given ChartJS coordinate is valid
#[derive(Debug)]
pub enum CoordinateError {
    InvalidX {
        ty: &'static str,
        error: Box<dyn Error + Sync + Send + 'static>,
        input: String,
    },
    InvalidY {
        ty: &'static str,
        error: Box<dyn Error + Sync + Send + 'static>,
        input: serde_json::Number,
    },

    Deserialize {
        value: JsValue,
        error: serde_json::Error,
    },
    MissingKey {
        value: JsValue,
        key: String,
    },
    GetTypedKey {
        value: JsValue,
        key: String,
        ty: &'static str,
    },
}

impl Display for CoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoordinateError::InvalidX { ty, error, input } => write!(
                f,
                "Invalid X coordinate of type `{ty}`: `{input}`, parsing failed due to: {error:?}"
            ),
            CoordinateError::InvalidY { ty, error, input } => write!(
                f,
                "Invalid Y coordinate of type `{ty}`: `{input}`, parsing failed due to: {error:?}"
            ),
            CoordinateError::Deserialize { value, error } => {
                write!(f, "Error deserializing value `{value:?}`: {error:?}")
            }
            CoordinateError::MissingKey { value, key } => {
                write!(f, "Key `{key}` is missing from object `{value:?}`")
            }
            CoordinateError::GetTypedKey { value, key, ty } => write!(
                f,
                "Key `{key}` (of type {ty}) is missing form object `{value:?}`"
            ),
        }
    }
}

impl Error for CoordinateError {}

/// A representation in rust of a ChartJS coordinate, allowing convenient formatting when constructing tooltips
#[derive(Debug)]
#[non_exhaustive]
pub struct Coordinate<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, TE, U, UE> Coordinate<T, U>
where
    T: FromStr<Err = TE>,
    U: FromStr<Err = UE>,
    TE: std::error::Error + Sync + Send + 'static,
    UE: std::error::Error + Sync + Send + 'static,
{
    fn from_raw(coord: Coordinate_) -> Result<Coordinate<T, U>, CoordinateError> {
        Ok(Coordinate {
            x: coord.x.parse().map_err(|e| CoordinateError::InvalidX {
                ty: type_name::<T>(),
                error: Box::new(e),
                input: coord.x.to_string(),
            })?,
            y: coord
                .y
                .to_string()
                .parse()
                .map_err(|e| CoordinateError::InvalidY {
                    ty: type_name::<U>(),
                    error: Box::new(e),
                    input: coord.y.clone(),
                })?,
        })
    }

    /// This should be used inside a `#[wasm_bindgen]` function
    /// where it is known that the parameter passed to the function
    /// will have the shape of an individual coordinate
    ///
    /// 1. first create a function
    ///
    /// ```rust no_run
    /// #[wasm_bindgen]
    /// pub fn chart_data_label_formatter(a: JsValue, _: JsValue) -> JsValue {
    ///     match Coordinate::<NaiveDate, Dollars>::from_js_value(a) {
    ///         Ok(val) => JsValue::from_str(&val.y.format().to_string()),
    ///         Err(e) => {
    ///             console_dbg!("Error converting to serde", e);
    ///             JsValue::from_str("")
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// 2. then use it with the [`crate::objects::DataLabels`] builder:
    ///
    /// ```rust no_run
    /// DataLabels::new()
    /// .formatter(FnWithArgs::<2>::new().run_rust_fn(chart_data_label_formatter)),
    /// ```
    pub fn from_js_value(val: JsValue) -> Result<Self, CoordinateError> {
        JsValueSerdeExt::into_serde::<Coordinate_>(&val)
            .map_err(|error| CoordinateError::Deserialize {
                value: val.clone(),
                error,
            })
            .and_then(Coordinate::<T, U>::from_raw)
    }
}

#[derive(serde::Deserialize)]
struct Coordinate_ {
    x: String,
    y: serde_json::Number,
}

/// A representation in rust of a ChartJS poinrt, generally exposed via the tooltips plugin
#[derive(Debug)]
#[non_exhaustive]
pub struct ChartJsPoint<T, U> {
    /// Probably don't use this
    pub formatted_value: String,
    /// Probably don't use this
    pub label: String,
    /// Details about the dataset that are seen by the viewer of the chart
    pub dataset: ChartJsPointDataset,
    /// The raw coordinate value for the point
    pub raw: Coordinate<T, U>,
}

fn get_field(val: &JsValue, key: &str) -> Result<JsValue, CoordinateError> {
    match Reflect::get(val, &JsValue::from_str(key)) {
        Ok(v) => Ok(v),
        Err(_) => Err(CoordinateError::MissingKey {
            value: val.clone(),
            key: key.to_string(),
        }),
    }
}

fn get_string(val: &JsValue, key: &str) -> Result<String, CoordinateError> {
    get_field(val, key)?
        .as_string()
        .ok_or_else(|| CoordinateError::GetTypedKey {
            value: val.clone(),
            key: key.to_string(),
            ty: "String",
        })
}

fn get_f64(val: &JsValue, key: &str) -> Result<f64, CoordinateError> {
    get_field(val, key)?
        .as_f64()
        .ok_or_else(|| CoordinateError::GetTypedKey {
            value: val.clone(),
            key: key.to_string(),
            ty: "f64",
        })
}

impl<T, TE, U, UE> ChartJsPoint<T, U>
where
    T: FromStr<Err = TE>,
    U: FromStr<Err = UE>,
    TE: std::error::Error + Sync + Send + 'static,
    UE: std::error::Error + Sync + Send + 'static,
    T: fmt::Debug,
    U: fmt::Debug,
{
    /// This should be used for parameter of [`crate::objects::TooltipCallbacks::label`]
    ///
    /// 1. first create a function
    ///
    ///  ```rust no_run
    /// #[wasm_bindgen]
    /// pub fn tooltip_value_callback(context: JsValue) -> JsValue {
    ///     match ChartJsPoint::<NaiveDate, Dollars>::parse(context) {
    ///         Ok(val) => {
    ///             let label = val.dataset.label;
    ///             let y = val.raw.y.format();
    ///             JsValue::from_str(&format!("{label}: {y}"))
    ///         }
    ///         Err(e) => {
    ///             console_dbg!("Error parsing", e);
    ///             JsValue::from_str("")
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// 2. then use with the builder:
    ///
    /// ```rust no_run
    /// ChartOptions::new()
    /// .plugins(
    ///     ChartPlugins::new()
    ///         .tooltip(
    ///             TooltipPlugin::new().callbacks(
    ///                 TooltipCallbacks::new()
    ///                     .label(
    ///                         FnWithArgs::<1>::new()
    ///                             .run_rust_fn(tooltip_value_callback),
    ///                     )
    ///             ),
    ///         ),
    /// )
    /// ```
    pub fn parse(val: JsValue) -> Result<Self, CoordinateError> {
        Ok(ChartJsPoint {
            formatted_value: get_string(&val, "formattedValue")?,
            label: get_string(&val, "label")?,
            dataset: JsValueSerdeExt::into_serde::<ChartJsPointDataset>(&get_field(
                &val, "dataset",
            )?)
            .map_err(|error| CoordinateError::Deserialize {
                value: val.clone(),
                error,
            })?,
            raw: Coordinate::from_js_value(get_field(&val, "raw")?)?,
        })
    }

    /// This should be used for the parameter of [`crate::objects::TooltipCallbacks::title`]
    ///
    /// 1. first create a function
    ///
    /// ```rust no_run
    /// #[wasm_bindgen]
    /// pub fn tooltip_date_title_callback(context: JsValue) -> JsValue {
    ///     match ChartJsPoint::<NaiveDate, Dollars>::parse_array(context) {
    ///         Ok(vals) if !vals.is_empty() => JsValue::from_str(vals[0].label.split(",").next().unwrap_or_default()),
    ///         Ok(_) => {
    ///             console_dbg!("Empty array");
    ///             JsValue::from_str("")
    ///         }
    ///         Err(e) => {
    ///             console_dbg!("Error parsing", e);
    ///             JsValue::from_str("")
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// 2. then use with the builder:
    ///
    /// ```rust no_run
    /// ChartOptions::new()
    /// .plugins(
    ///     ChartPlugins::new()
    ///         .tooltip(
    ///             TooltipPlugin::new().callbacks(
    ///                 TooltipCallbacks::new()
    ///                     .title(
    ///                         FnWithArgs::<1>::new()
    ///                             .run_rust_fn(tooltip_date_title_callback),
    ///                     )
    ///             ),
    ///         ),
    /// )
    /// ```
    pub fn parse_array(val: JsValue) -> Result<Vec<Self>, CoordinateError> {
        let vec = if val.is_array() {
            let array = val.dyn_into::<Array>().unwrap_or_default().to_vec();
            let mut parsed = Vec::new();
            for item in array {
                parsed.push(Self::parse(item)?);
            }
            parsed
        } else {
            Vec::from([Self::parse(val)?])
        };
        Ok(vec)
    }
}

#[derive(serde::Deserialize)]
struct ChartJsPoint_ {
    formatted_value: String,
    label: String,
    dataset: ChartJsPointDataset,
    raw: Coordinate_,
}

#[derive(serde::Deserialize, Debug)]
pub struct ChartJsPointDataset {
    /// The label for the dataset that is seen by the viewer of the chart
    pub label: String,
    pub r#type: String,
}
