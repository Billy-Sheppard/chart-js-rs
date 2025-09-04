#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]

pub mod bar;
pub mod coordinate;
pub mod doughnut;
pub mod exports;
pub mod functions;
pub mod objects;
pub mod pie;
pub mod scatter;
pub mod traits;
pub mod worker;

#[doc(hidden)]
mod utils;

use exports::get_chart;
use gloo_utils::format::JsValueSerdeExt;
pub use objects::*;
use serde::{de::DeserializeOwned, Serialize};
pub use traits::*;
pub use utils::*;

pub trait ChartExt: DeserializeOwned + Serialize + Default {
    type DS;

    fn new(id: impl AsRef<str>) -> Self {
        Self::default().id(id.as_ref().into())
    }

    fn get_id(&self) -> &str;
    fn id(self, id: String) -> Self;

    fn get_data(&mut self) -> &mut Self::DS;
    fn data(mut self, data: impl Into<Self::DS>) -> Self {
        *self.get_data() = data.into();
        self
    }

    fn get_options(&mut self) -> &mut ChartOptions;
    fn options(mut self, options: impl Into<ChartOptions>) -> Self {
        *self.get_options() = options.into();
        self
    }

    fn into_chart(self) -> Chart {
        Chart {
            obj: <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self)
                .expect("Unable to serialize chart."),
            id: self.get_id().into(),
            mutate: false,
            plugins: String::new(),
            defaults: String::new(),
            #[cfg(feature = "workers")]
            worker: None,
        }
    }

    #[cfg(feature = "workers")]
    #[allow(async_fn_in_trait)]
    async fn into_worker_chart(self) -> Result<Chart, Box<dyn std::error::Error>> {
        Ok(Chart {
            obj: <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self)
                .expect("Unable to serialize chart."),
            id: self.get_id(),
            mutate: false,
            plugins: String::new(),
            defaults: String::new(),
            worker: Some(crate::worker::ChartWorker::new().await?),
        })
    }

    fn get_chart_from_id(id: &str) -> Option<Self> {
        let chart = get_chart(id);

        serde_wasm_bindgen::from_value(chart)
            .inspect_err(|e| {
                gloo_console::error!(e.to_string());
            })
            .ok()
    }
}
