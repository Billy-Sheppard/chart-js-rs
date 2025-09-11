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

#[cfg(feature = "workers")]
pub mod worker;

pub use objects::*;
pub use traits::*;
pub use utils::*;

#[cfg(feature = "workers")]
pub use worker_chart::*;

#[doc(hidden)]
mod utils;

use exports::get_chart;
use gloo_utils::format::JsValueSerdeExt;
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "workers")]
use wasm_bindgen::{self, prelude::*};

#[cfg(feature = "workers")]
use web_sys::WorkerGlobalScope;

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
        }
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

#[cfg(feature = "workers")]
pub fn is_worker() -> bool {
    js_sys::global().dyn_into::<WorkerGlobalScope>().is_ok()
}

#[cfg(feature = "workers")]
mod worker_chart {
    use crate::*;

    pub trait WorkerChartExt: ChartExt {
        #[allow(async_fn_in_trait)]
        async fn into_worker_chart(
            self,
            imports_block: &str,
        ) -> Result<WorkerChart, Box<dyn std::error::Error>> {
            Ok(WorkerChart {
                obj: <::wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&self)
                    .expect("Unable to serialize chart."),
                id: self.get_id().into(),
                mutate: false,
                plugins: String::new(),
                defaults: String::new(),
                worker: crate::worker::ChartWorker::new(imports_block).await?,
            })
        }
    }
    #[wasm_bindgen]
    #[derive(Clone)]
    #[must_use = "\nAppend .render_async()\n"]
    pub struct WorkerChart {
        pub(crate) obj: JsValue,
        pub(crate) id: String,
        pub(crate) mutate: bool,
        pub(crate) plugins: String,
        pub(crate) defaults: String,
        pub(crate) worker: crate::worker::ChartWorker,
    }
    impl WorkerChart {
        pub async fn render_async(self) -> Result<(), Box<dyn std::error::Error>> {
            self.worker
                .render(self.obj, &self.id, self.mutate, self.plugins, self.defaults)
                .await
        }

        pub async fn update_async(self, animate: bool) -> Result<bool, Box<dyn std::error::Error>> {
            self.worker.update(self.obj, &self.id, animate).await
        }

        #[must_use = "\nAppend .render_async()\n"]
        pub fn mutate(&mut self) -> Self {
            self.mutate = true;
            self.clone()
        }

        #[must_use = "\nAppend .render_async()\n"]
        pub fn plugins(&mut self, plugins: impl Into<String>) -> Self {
            self.plugins = plugins.into();
            self.clone()
        }

        #[must_use = "\nAppend .render_async()\n"]
        pub fn defaults(&mut self, defaults: impl Into<String>) -> Self {
            self.defaults = format!("{}\n{}", self.defaults, defaults.into());
            self.to_owned()
        }
    }
}
