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
pub use worker::{ChartWorker, DEFAULT_WORKER_IMPORTS};
#[cfg(feature = "workers")]
pub use worker_chart::*;
#[cfg(feature = "workers")]
pub use worxide::is_worker;

#[doc(hidden)]
mod utils;

use exports::get_chart;
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
        // Serialize via serde_json::Value then JSON-parse to a JsValue. The
        // serde-wasm-bindgen shortcut (Rust -> JsValue directly) is faster but
        // changes semantics Chart.js is sensitive to (`None` -> `undefined`
        // instead of `null`, different map/number handling), which collapses
        // some charts — so keep this round-trip.
        let json_value = erased_serde::serialize(self, serde_json::value::Serializer)
            .expect("Unable to serialize chart!");
        <wasm_bindgen::JsValue as gloo_utils::format::JsValueSerdeExt>::from_serde(&json_value)
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
    use std::cell::RefCell;
    use std::collections::HashMap;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    /// Object-safe interface for staging a chart on a worker.
    ///
    /// The two vtable methods are all the worker needs: serialize the chart (on
    /// the worker) and read its canvas id. They take `&self` and return
    /// non-`Self` types, so `dyn WorkerChartExt` is valid — you can hold a
    /// `Box<dyn WorkerChartExt>` and render heterogeneous charts uniformly.
    /// `into_worker_chart` is `where Self: Sized`, so it stays callable on
    /// concrete charts without affecting object safety.
    ///
    /// Blanket-implemented for every [`ChartExt`] chart.
    pub trait WorkerChartExt: Send + 'static {
        /// Serialize the chart to a Chart.js config. Runs on the worker.
        fn render_json(&self) -> wasm_bindgen::JsValue;

        /// The chart's canvas element id.
        fn chart_id(&self) -> String;

        /// Boot a worker (importing `libs`) and stage this chart on it.
        ///
        /// The `self: Box<Self>` receiver is object-safe, so this is callable
        /// through a `dyn WorkerChartExt` (e.g. on a `Box<dyn WorkerChartExt>`)
        /// while still moving the chart to the worker by pointer. `imports` is the JS
        /// boot block the worker runs (use [`DEFAULT_WORKER_IMPORTS`] for the
        /// usual Chart.js + Luxon setup). For a worker shared across charts, construct a
        /// [`ChartWorker`] yourself and use [`WorkerChart::on`].
        #[allow(clippy::type_complexity)]
        fn into_worker_chart(
            self: Box<Self>,
            imports: String,
        ) -> Pin<Box<dyn Future<Output = Result<WorkerChart, Box<dyn Error>>>>>;
    }

    impl<T: ChartExt + Send + 'static> WorkerChartExt for T {
        fn render_json(&self) -> wasm_bindgen::JsValue {
            self.into_json()
        }
        fn chart_id(&self) -> String {
            self.get_id().to_string()
        }
        // Body lives here (not as a trait default) because the
        // `Box<Self> -> Box<dyn WorkerChartExt>` coercion needs `Self: Sized`,
        // which holds for the concrete `T` but not in a `?Sized` default-method
        // body. The trait declaration stays defaultless so the method remains in
        // the vtable and is callable through `dyn WorkerChartExt`.
        #[allow(clippy::type_complexity)]
        fn into_worker_chart(
            self: Box<Self>,
            imports: String,
        ) -> Pin<Box<dyn Future<Output = Result<WorkerChart, Box<dyn Error>>>>> {
            Box::pin(async move {
                let worker = ChartWorker::with_imports(imports).await?;
                Ok(WorkerChart::on(worker, self))
            })
        }
    }

    /// Methods available directly on the trait object (`dyn WorkerChartExt` /
    /// `Box<dyn WorkerChartExt>`), where the non-object-safe `ChartExt`
    /// supertrait can't be required. Inherent methods, so they don't collide
    /// with the equivalently-named `ChartExt` methods on concrete charts.
    impl dyn WorkerChartExt {
        /// Build a main-thread [`Chart`] from this worker chart. Mirrors
        /// [`ChartExt::into_chart`].
        #[allow(clippy::wrong_self_convention)]
        pub fn into_chart(&self) -> Chart {
            Chart {
                obj: self.render_json(),
                id: self.chart_id(),
                mutate: false,
                plugins: String::new(),
                defaults: String::new(),
            }
        }
    }

    /// A chart staged on a worker, awaiting `render_async`. Holds the chart as a
    /// trait object, so it is not generic over the chart type.
    #[must_use = "\nAppend .render_async()\n"]
    pub struct WorkerChart {
        chart: Box<dyn WorkerChartExt>,
        worker: ChartWorker,
        plugins: String,
        defaults: String,
        /// Optional loading hook. Called once at render start with the canvas's
        /// parent element; any DOM the callback appends to that parent is
        /// removed automatically when the render finishes (success or error).
        while_rendering: Option<Box<dyn FnOnce(web_sys::HtmlElement)>>,
        /// Optional worker-side setup, run after the worker's libraries load and
        /// before the chart is built. Use it to register custom Chart.js plugins
        /// on the worker and to move owned Rust state across (captured in `f`).
        worker_setup: Option<Box<dyn FnOnce() + Send + 'static>>,
    }

    thread_local! {
        /// Live chart workers keyed by canvas id. A chart's worker is persistent
        /// — it owns the OffscreenCanvas + Chart.js instance and serves tooltips
        /// and updates — so it must outlive `render_async` (which otherwise drops
        /// the only `Rc`, and worxide's `Worker` terminates on drop). We keep it
        /// here and tear it down when the canvas element leaves the DOM (component
        /// unmount, SPA navigation), so the consumer never has to manage it.
        static LIVE: RefCell<HashMap<String, LiveChart>> = RefCell::new(HashMap::new());

        /// Ids whose first render is in flight. `render_async` is async, so the
        /// `LIVE` entry isn't recorded until after the render await — two calls
        /// fired for one mount would both see `LIVE` empty and both transfer.
        /// This set is claimed synchronously (before any await) so the second
        /// call sees the claim and reuses instead of transferring again.
        static PENDING: RefCell<std::collections::HashSet<String>> =
            RefCell::new(std::collections::HashSet::new());
    }

    struct LiveChart {
        worker: ChartWorker,
        /// The canvas element this worker's OffscreenCanvas was transferred
        /// from. Used to tell "rendered again on the same node" (→ update) from
        /// "node recreated with the same id" (→ fresh transfer).
        el: web_sys::Element,
        observer: web_sys::MutationObserver,
        _on_mutation: Closure<dyn FnMut()>,
        /// Resize/zoom watchers; dropped (torn down) with this entry.
        _resize: crate::worker::ResizeWatchers,
        /// DOM mouse-forwarding listeners; dropped (removed + freed) with this
        /// entry rather than leaked.
        _mouse: crate::worker::MouseHandlers,
    }

    /// Terminate and forget the live worker for `id`, if any.
    fn teardown(id: &str) {
        PENDING.with(|p| p.borrow_mut().remove(id));
        if let Some(live) = LIVE.with(|m| m.borrow_mut().remove(id)) {
            live.observer.disconnect();
            live.worker.terminate();
            // observer + closure are dropped here (off any observer callback).
        }
    }

    /// Keep `worker` alive until canvas `el` is removed from the DOM, then
    /// terminate it. Watches the document subtree and tears down once the
    /// element is no longer connected.
    /// Resolve on the next animation frame. Used to defer removing the loading
    /// hook's DOM until the just-rendered chart has composited to screen, so a
    /// transparent chart never shows the spinner through it after render.
    async fn next_animation_frame() {
        let Some(win) = web_sys::window() else {
            return;
        };
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let cb = Closure::once_into_js(move |_t: wasm_bindgen::JsValue| {
                let _ = resolve.call0(&wasm_bindgen::JsValue::NULL);
            });
            let _ = win.request_animation_frame(cb.unchecked_ref());
        });
        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
    }

    fn keep_until_removed(
        id: String,
        el: web_sys::Element,
        worker: ChartWorker,
        mouse: crate::worker::MouseHandlers,
    ) {
        let cb = {
            let id = id.clone();
            let el = el.clone();
            Closure::<dyn FnMut()>::new(move || {
                if !el.is_connected() {
                    // Defer the map mutation off the observer's own callback so
                    // the closure isn't dropped while it is executing.
                    let id = id.clone();
                    wasm_bindgen_futures::spawn_local(async move { teardown(&id) });
                }
            })
        };
        let observer = match web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
            Ok(o) => o,
            Err(_) => return, // no observer available; skip auto-teardown
        };
        let resize = worker.install_resize_watchers(&el, &id);
        if let Some(body) = gloo_utils::document().body() {
            let init = js_sys::Object::new();
            let _ = js_sys::Reflect::set(&init, &"childList".into(), &wasm_bindgen::JsValue::TRUE);
            let _ = js_sys::Reflect::set(&init, &"subtree".into(), &wasm_bindgen::JsValue::TRUE);
            if let Err(e) = observer.observe_with_options(&body, init.unchecked_ref()) {
                gloo_console::warn!(format!(
                    "chart-js-rs: MutationObserver.observe failed; auto-teardown disabled \
                     for this chart: {e:?}"
                ));
            }
        }
        LIVE.with(|m| {
            m.borrow_mut().insert(
                id,
                LiveChart {
                    worker,
                    el,
                    observer,
                    _on_mutation: cb,
                    _resize: resize,
                    _mouse: mouse,
                },
            );
        });
    }

    impl WorkerChart {
        /// Stage a chart on an existing (possibly shared) worker. A concrete
        /// chart is boxed at the call site: `WorkerChart::on(worker, Box::new(chart))`
        /// — or just use [`WorkerChartExt::into_worker_chart`].
        pub fn on(worker: ChartWorker, chart: Box<dyn WorkerChartExt>) -> Self {
            Self {
                chart,
                worker,
                plugins: String::new(),
                defaults: String::new(),
                while_rendering: None,
                worker_setup: None,
            }
        }

        /// Render the chart on the worker. Consumes the builder; the chart moves
        /// to the worker by pointer and is serialized there.
        pub async fn render_async(mut self) -> Result<(), Box<dyn Error>> {
            let id = self.chart.chart_id();

            // A canvas can be transferred to a worker exactly once. If this id is
            // already live *on the same DOM node* (e.g. `render_async` ran twice
            // for one mount), don't transfer again — reuse the existing worker
            // and update it in place.
            let target = gloo_utils::document().get_element_by_id(&id);
            let reuse = LIVE.with(|m| {
                let m = m.borrow();
                let live = m.get(&id)?;
                let target = target.as_ref()?;
                (live.el.is_connected() && js_sys::Object::is(live.el.as_ref(), target.as_ref()))
                    .then(|| live.worker.clone())
            });
            if let Some(worker) = reuse {
                return worker.update(self.chart, id, true).await.map(|_| ());
            }

            // Race guard: two render_async calls for the same id can both pass
            // the reuse check above before either registers in LIVE. The first
            // to reach `transfer_canvas` marks the element synchronously (before
            // it yields), so if the attribute is already present, a concurrent
            // call owns this chart — exit cleanly rather than transferring twice.
            if target
                .as_ref()
                .map(|t| t.has_attribute("data-cjsrs-transferred"))
                .unwrap_or(false)
            {
                return Ok(());
            }

            // Synchronously claim this id before any await. `render_async` is
            // async and doesn't record `LIVE` until after the render completes,
            // so without this two concurrent calls for one mount would both see
            // `LIVE` empty and both transfer the canvas (the second throws). If
            // the claim is already held, a render is in flight on this node —
            // drop this duplicate call rather than transfer again.
            let claimed = PENDING.with(|p| p.borrow_mut().insert(id.clone()));
            if !claimed {
                return Ok(());
            }

            // New, or a recreated node under a reused id: clear any stale live
            // entry, then render (which transfers the fresh canvas exactly once).
            // Note: `teardown` clears PENDING, so re-insert the claim after it.
            if LIVE.with(|m| m.borrow().contains_key(&id)) {
                teardown(&id);
                PENDING.with(|p| p.borrow_mut().insert(id.clone()));
            }

            // Loading hook: hand the consumer the canvas's parent, snapshot the
            // children that already exist, and let them append a spinner. After
            // the render we remove only the nodes that appeared in between, so
            // the canvas and any pre-existing overlays survive.
            let loading: Option<(web_sys::Element, Vec<web_sys::Node>)> = {
                let parent = gloo_utils::document()
                    .get_element_by_id(&id)
                    .and_then(|el| el.parent_element());
                match (self.while_rendering.take(), parent) {
                    (Some(cb), Some(parent)) => {
                        let kids = parent.child_nodes();
                        let mut before = Vec::with_capacity(kids.length() as usize);
                        for i in 0..kids.length() {
                            if let Some(n) = kids.item(i) {
                                before.push(n);
                            }
                        }
                        if let Ok(he) = parent.clone().dyn_into::<web_sys::HtmlElement>() {
                            cb(he);
                        }
                        Some((parent, before))
                    }
                    _ => None,
                }
            };

            // Worker-side setup (plugin registration etc.) runs after bootstrap
            // and before the chart is built, so plugins are registered on the
            // worker's `Chart` before construction (Chart.js applies plugins and
            // runs their `beforeInit` at construct time).
            if let Some(setup) = self.worker_setup.take() {
                if let Err(e) = self.worker.run_setup(setup).await {
                    PENDING.with(|p| p.borrow_mut().remove(&id));
                    return Err(e);
                }
            }

            let worker = self.worker.clone();
            let result = self
                .worker
                .render(self.chart, id.clone(), self.plugins, self.defaults)
                .await;

            // Release the in-flight claim regardless of outcome.
            PENDING.with(|p| p.borrow_mut().remove(&id));

            // Remove whatever the loading hook appended (success or error).
            // Wait one animation frame first so the just-rendered chart has
            // composited to screen before the hook's DOM is removed — otherwise
            // a transparent chart briefly shows the spinner through it.
            if let Some((parent, before)) = loading {
                next_animation_frame().await;
                let kids = parent.child_nodes();
                let mut added = Vec::new();
                for i in 0..kids.length() {
                    if let Some(n) = kids.item(i) {
                        if !before
                            .iter()
                            .any(|b| js_sys::Object::is(b.as_ref(), n.as_ref()))
                        {
                            added.push(n);
                        }
                    }
                }
                for n in added {
                    let _ = parent.remove_child(&n);
                }
            }

            // Propagate a render error (the mouse handle was already dropped on
            // the error path inside `ChartWorker::render`); otherwise take it.
            let mouse = result?;

            // Keep the worker alive (for tooltips / updates) until the canvas
            // element leaves the DOM, then terminate it. The mouse handle rides
            // along so its listeners are removed on teardown rather than leaked.
            if let Some(el) = gloo_utils::document().get_element_by_id(&id) {
                keep_until_removed(id, el, worker, mouse);
            }
            Ok(())
        }

        /// Update a chart previously rendered on this worker.
        pub async fn update_async(self, animate: bool) -> Result<bool, Box<dyn Error>> {
            let id = self.chart.chart_id();
            self.worker.update(self.chart, id, animate).await
        }

        /// Run a closure ON THE WORKER after its libraries load and before the
        /// chart is built. This is where you register custom Chart.js plugins on
        /// the worker's `Chart` global. Owned Rust state captured by `f` (e.g.
        /// an `Arc` of resolved data, or a synced `Mutable`) moves to the worker
        /// with the closure. Runs once, on a fresh render (not on in-place
        /// updates, where the worker already has its plugins).
        #[must_use = "\nAppend .render_async()\n"]
        pub fn on_worker_setup<F>(mut self, f: F) -> Self
        where
            F: FnOnce() + Send + 'static,
        {
            self.worker_setup = Some(Box::new(f));
            self
        }

        /// Show a loading indicator while the worker renders.
        ///
        /// `f` is called once, at render start, with the canvas's parent
        /// element; append your spinner (or anything) to it. Whatever you add
        /// is removed automatically once the chart is built — including on
        /// error, so it can't get stuck. Pre-existing children (the canvas, your
        /// own overlays) are left untouched. Only fires on a fresh render, not
        /// on in-place updates.
        #[must_use = "\nAppend .render_async()\n"]
        pub fn while_rendering(mut self, f: impl FnOnce(web_sys::HtmlElement) + 'static) -> Self {
            self.while_rendering = Some(Box::new(f));
            self
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
