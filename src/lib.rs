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
pub use worxide::is_worker;
#[cfg(feature = "workers")]
pub use worker::{ChartWorker, WorkerLibs};
#[cfg(feature = "workers")]
pub use worker_chart::*;

#[doc(hidden)]
mod utils;

use exports::get_chart;
use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;

pub trait ChartExt: erased_serde::Serialize {
    type DS;

    fn new(id: impl AsRef<str>) -> Self
    where
        Self: Default,
    {
        Self::default().id(id.as_ref().into())
    }

    fn get_id(&self) -> &str;
    fn id(self, id: String) -> Self
    where
        Self: Sized;

    fn get_data(&mut self) -> &mut Self::DS;
    fn data(mut self, data: impl Into<Self::DS>) -> Self
    where
        Self: Sized,
    {
        *self.get_data() = data.into();
        self
    }

    fn get_options(&mut self) -> &mut ChartOptions;
    fn options(mut self, options: impl Into<ChartOptions>) -> Self
    where
        Self: Sized,
    {
        *self.get_options() = options.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    fn into_json(&self) -> wasm_bindgen::JsValue {
        let json_value = erased_serde::serialize(self, serde_json::value::Serializer)
            .expect("Unable to serialize chart!");
        <wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&json_value)
            .expect("Unable to convert to JsValue!")
    }

    #[allow(clippy::wrong_self_convention)]
    fn into_chart(&self) -> Chart {
        Chart {
            obj: self.into_json(),
            id: self.get_id().into(),
            mutate: false,
            plugins: String::new(),
            defaults: String::new(),
        }
    }

    fn get_chart_from_id(id: &str) -> Option<Self>
    where
        for<'de> Self: Deserialize<'de>,
    {
        let chart = get_chart(id);

        serde_wasm_bindgen::from_value(chart)
            .inspect_err(|e| {
                gloo_console::error!(e.to_string());
            })
            .ok()
    }
}

#[cfg(feature = "workers")]
mod worker_chart {
    use std::{error::Error, future::Future, pin::Pin};

    use crate::worker::ChartWorker;
    use crate::*;

    /// Builds a [`WorkerChart`] from a chart. The chart is kept as a Rust value
    /// and only serialized once it reaches the worker (`render_async`), so the
    /// dataset never serializes on the main thread.
    pub trait WorkerChartExt: ChartExt + Send + 'static + Sized {
        /// Boot a dedicated worker (default JS lib URLs) and stage this chart on
        /// it. For a shared worker across many charts, construct a
        /// [`ChartWorker`] yourself and use [`WorkerChart::on`].
        #[allow(clippy::type_complexity)]
        fn into_worker_chart(
            self,
        ) -> Pin<Box<dyn Future<Output = Result<WorkerChart<Self>, Box<dyn Error>>>>> {
            Box::pin(async move {
                let worker = ChartWorker::new().await?;
                Ok(WorkerChart::on(worker, self))
            })
        }
    }

    /// A chart staged on a worker, awaiting `render_async`. Generic over the
    /// concrete chart type so the chart can ride to the worker by pointer.
    #[must_use = "\nAppend .render_async()\n"]
    pub struct WorkerChart<C: ChartExt> {
        chart: C,
        worker: ChartWorker,
        plugins: String,
        defaults: String,
    }

    impl<C: ChartExt + Send + 'static> WorkerChart<C> {
        /// Stage `chart` on an existing (possibly shared) worker.
        pub fn on(worker: ChartWorker, chart: C) -> Self {
            Self {
                chart,
                worker,
                plugins: String::new(),
                defaults: String::new(),
            }
        }

        /// Render the chart on the worker. Consumes the builder; the chart moves
        /// to the worker by pointer and is serialized there.
        pub async fn render_async(self) -> Result<(), Box<dyn Error>> {
            let id = self.chart.get_id().to_string();
            self.worker
                .render(self.chart, id, self.plugins, self.defaults)
                .await
        }

        /// Update a chart previously rendered on this worker.
        pub async fn update_async(self, animate: bool) -> Result<bool, Box<dyn Error>> {
            let id = self.chart.get_id().to_string();
            self.worker.update(self.chart, id, animate).await
        }

        #[must_use = "\nAppend .render_async()\n"]
        pub fn plugins(mut self, plugins: impl Into<String>) -> Self {
            self.plugins = plugins.into();
            self
        }

        #[must_use = "\nAppend .render_async()\n"]
        pub fn defaults(mut self, defaults: impl Into<String>) -> Self {
            self.defaults = format!("{}\n{}", self.defaults, defaults.into());
            self
        }
    }
}
