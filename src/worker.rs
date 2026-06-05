//! Off-main-thread Chart.js rendering, built on `worxide`.
//!
//! The worker is a persistent `worxide::Worker`: it boots once, attaches to the
//! app's shared wasm memory, and thereafter runs Rust closures we hand it. A
//! chart crosses to the worker as a Rust value (captured in that closure, moved
//! by pointer), so `ChartExt::into_json()` — the serialization — runs on the
//! worker, and the big serialized config is never structured-cloned across
//! `postMessage`. The main thread only ever transfers the `OffscreenCanvas` and
//! forwards pointer-light mouse events.
//!
//! JavaScript surface: this module keeps almost nothing in JS. The chart
//! building (`derationalize`, plugin/defaults `eval`, `new Chart`, config swap,
//! instance registry, listeners) is all Rust via `js-sys`. The two irreducible
//! bits are Rust-constructed `Function`s (no `.js` files):
//!   * a one-line dynamic-`import()` wrapper (`import` is syntax, not callable
//!     by reference), and
//!   * the Chart.js mouse/tooltip/legend interaction, which pokes Chart.js
//!     internals and is genuinely clearer as ~50 lines of JS than as `Reflect`.
//!
//! Everything else is Rust. `worker_shim.js` / `worker_imports.js` are gone.

use crate::ChartExt;
use js_sys::{Array, Function, Object, Promise, Reflect};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// CDN URLs for the worker's JS dependencies (defaults match the example's
/// versions). Override via [`ChartWorker::with_libs`].
#[derive(Clone, Debug)]
pub struct WorkerLibs {
    pub chart_js: String,
    pub luxon: String,
    pub luxon_adapter: String,
}
impl Default for WorkerLibs {
    fn default() -> Self {
        Self {
            chart_js: "https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.js".into(),
            luxon: "https://cdn.jsdelivr.net/npm/luxon@^2/+esm".into(),
            luxon_adapter:
                "https://cdn.jsdelivr.net/npm/chartjs-adapter-luxon@^1/dist/chartjs-adapter-luxon.umd.min.js"
                    .into(),
        }
    }
}

// ===========================================================================
// Worker side  — runs on the worker, via run/run_blocking closures or the
// message listener installed at bootstrap.
// ===========================================================================

struct CanvasEntry {
    canvas: JsValue,
    width: f64,
    height: f64,
}

thread_local! {
    /// OffscreenCanvases transferred in, keyed by chart id, awaiting `render`.
    static CANVASES: RefCell<HashMap<String, CanvasEntry>> = RefCell::new(HashMap::new());
    /// Live Chart.js instances, keyed by chart id.
    static CHARTS: RefCell<HashMap<String, JsValue>> = RefCell::new(HashMap::new());

    /// `url => import(url)`. The single irreducible line of JS: `import` is
    /// syntax and can't be obtained by reference, so we wrap it once.
    static IMPORT_FN: Function = Function::new_with_args("url", "return import(url);");

    /// Chart.js mouse/tooltip/legend interaction, kept as JS because doing it
    /// through `Reflect` would be far longer and more brittle. Args:
    /// (chart, eventType, x, y, computedStyles).
    static MOUSE_FN: Function = Function::new_with_args(
        "chart, eventType, x, y, computedStyles",
        r#"
        if (!chart) return;
        if (computedStyles && !chart.canvas.ownerDocument) {
            chart.canvas.ownerDocument = { defaultView: { getComputedStyle: function () {
                computedStyles.getPropertyValue = function (prop) {
                    const camel = prop.replace(/-([a-z])/g, (m, l) => l.toUpperCase());
                    return computedStyles[camel] || computedStyles[prop];
                };
                return computedStyles;
            } } };
        }
        const ev = {
            type: eventType, x, y, offsetX: x, offsetY: y,
            target: chart.canvas, currentTarget: chart.canvas,
            preventDefault() {}, stopPropagation() {},
        };
        const mode = chart.options.interaction?.mode || 'nearest';
        const opts = chart.options.interaction || { intersect: false };
        if (eventType === 'mousemove') {
            chart.tooltip.setActiveElements(
                chart.getElementsAtEventForMode(ev, mode, opts, false), ev);
            chart.draw();
        } else if (eventType === 'mouseleave') {
            chart.tooltip.setActiveElements([], ev);
            chart.draw();
        } else if (eventType === 'click') {
            const legend = chart.legend;
            if (legend && legend.legendHitBoxes) {
                for (let i = 0; i < legend.legendHitBoxes.length; i++) {
                    const h = legend.legendHitBoxes[i];
                    if (x >= h.left && x <= h.left + h.width && y >= h.top && y <= h.top + h.height) {
                        const meta = chart.getDatasetMeta(h.datasetIndex !== undefined ? h.datasetIndex : i);
                        if (meta) { meta.hidden = !meta.hidden; chart.update(); return; }
                    }
                }
            }
            const els = chart.getElementsAtEventForMode(ev, mode, opts, false);
            chart._handleEvent(ev);
            if (chart.options.onClick) chart.options.onClick(ev, els, chart);
        }
        "#,
    );
}

fn worker_global() -> web_sys::DedicatedWorkerGlobalScope {
    js_sys::global().unchecked_into()
}

/// `import(url)` as a future, via the wrapper Function.
async fn dyn_import(url: &str) -> Result<JsValue, JsValue> {
    let promise = IMPORT_FN.with(|f| f.call1(&JsValue::NULL, &JsValue::from_str(url)))?;
    JsFuture::from(Promise::from(promise)).await
}

/// Resolve the global `Chart` constructor (set by the Chart.js UMD import).
fn chart_ctor() -> Result<Function, JsValue> {
    Reflect::get(&js_sys::global(), &"Chart".into())?
        .dyn_into::<Function>()
        .map_err(|_| JsValue::from_str("global `Chart` is not a constructor — is Chart.js loaded?"))
}

/// Call `obj[name](...args)`.
fn call_method(obj: &JsValue, name: &str, args: &[JsValue]) -> Result<JsValue, JsValue> {
    let f = Reflect::get(obj, &name.into())?.dyn_into::<Function>()?;
    let arr = Array::new();
    for a in args {
        arr.push(a);
    }
    Reflect::apply(&f, obj, &arr)
}

fn is_plain_object(v: &JsValue) -> bool {
    v.is_object() && !Array::is_array(v) && !v.is_null()
}

/// Rebuild serialized closures (`{args, body, closure_id, return_value}`) into
/// real `Function`s, recursively — the Rust port of the JS `derationalize`.
///
/// NOTE: the `closure_id` branch preserves the original shim's placeholder
/// (`return 'orange'`) verbatim — a known pre-existing bug, not introduced
/// here; left as-is so this port changes nothing semantically.
fn derationalize(o: JsValue) -> JsValue {
    if Array::is_array(&o) {
        let out = Array::new();
        for item in Array::from(&o).iter() {
            out.push(&derationalize(item));
        }
        return out.into();
    }
    if !is_plain_object(&o) {
        return o;
    }
    let has = |k: &str| Reflect::has(&o, &k.into()).unwrap_or(false);
    if has("args") && has("body") && has("closure_id") && has("return_value") {
        let args = Reflect::get(&o, &"args".into()).unwrap_or(JsValue::UNDEFINED);
        let arg_names = Array::from(&args)
            .iter()
            .filter_map(|a| a.as_string())
            .collect::<Vec<_>>()
            .join(", ");
        let closure_id = Reflect::get(&o, &"closure_id".into()).unwrap_or(JsValue::UNDEFINED);
        let body_src = if closure_id.is_truthy() {
            " \n return 'orange'; \n ".to_string()
        } else {
            let body = Reflect::get(&o, &"body".into())
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_default();
            let ret = Reflect::get(&o, &"return_value".into())
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_default();
            format!(" {body} \n return {ret} ")
        };
        return Function::new_with_args(&arg_names, &body_src).into();
    }
    let out = Object::new();
    for entry in Object::entries(&o.clone().unchecked_into::<Object>()).iter() {
        let pair = Array::from(&entry);
        let _ = Reflect::set(&out, &pair.get(0), &derationalize(pair.get(1)));
    }
    out.into()
}

/// Build a fresh Chart.js instance on the transferred canvas. `obj` is the
/// chart config already serialized (on this worker) by `into_json`.
fn build_chart(
    entry: &CanvasEntry,
    mut obj: JsValue,
    plugins: &str,
    defaults: &str,
) -> Result<JsValue, JsValue> {
    if !defaults.is_empty() {
        // Side-effecting block (e.g. `Chart.defaults.*`); indirect-eval runs in
        // global scope, where `Chart` and `chartAreaBackground` live.
        js_sys::eval(defaults)?;
    }
    if !plugins.is_empty() {
        let plugins_val = js_sys::eval(plugins)?;
        Reflect::set(&obj, &"plugins".into(), &plugins_val)?;
    }

    obj = derationalize(obj);

    Reflect::set(
        &entry.canvas,
        &"width".into(),
        &JsValue::from_f64(entry.width),
    )?;
    Reflect::set(
        &entry.canvas,
        &"height".into(),
        &JsValue::from_f64(entry.height),
    )?;

    let chart = Reflect::construct(&chart_ctor()?, &Array::of2(&entry.canvas, &obj))?;
    let _ = call_method(&chart, "resize", &[]);

    // Initial animation unless options.animation === false.
    let animate = Reflect::get(&obj, &"options".into())
        .ok()
        .and_then(|o| Reflect::get(&o, &"animation".into()).ok())
        .map(|a| a != JsValue::FALSE)
        .unwrap_or(true);
    if animate {
        let _ = call_method(&chart, "update", &[JsValue::from_str("active")]);
    }
    Ok(chart)
}

/// Swap a live chart's config and update. Returns whether it succeeded.
fn update_chart(chart: &JsValue, updated: JsValue, animate: bool) -> bool {
    let go = || -> Result<(), JsValue> {
        let updated = derationalize(updated);
        let inner = Reflect::get(&Reflect::get(chart, &"config".into())?, &"_config".into())?;
        Reflect::set(
            &inner,
            &"type".into(),
            &Reflect::get(&updated, &"type".into())?,
        )?;
        Reflect::set(
            &inner,
            &"data".into(),
            &Reflect::get(&updated, &"data".into())?,
        )?;
        Reflect::set(
            &inner,
            &"options".into(),
            &Reflect::get(&updated, &"options".into())?,
        )?;
        if animate {
            call_method(chart, "update", &[])?;
            call_method(chart, "resize", &[])?;
        } else {
            call_method(chart, "update", &[JsValue::from_str("none")])?;
        }
        Ok(())
    };
    match go() {
        Ok(()) => true,
        Err(e) => {
            gloo_console::error!(format!("chart-js-rs:worker update failed: {e:?}"));
            false
        }
    }
}

/// Stash a transferred OffscreenCanvas (called by the message listener).
fn store_canvas(id: String, canvas: JsValue, width: f64, height: f64) {
    CANVASES.with(|c| {
        c.borrow_mut().insert(
            id,
            CanvasEntry {
                canvas,
                width,
                height,
            },
        );
    });
}

/// Route a forwarded mouse event to its chart's interaction handler.
fn handle_mouse(id: &str, event_type: &str, x: f64, y: f64, styles: JsValue) {
    CHARTS.with(|m| {
        if let Some(chart) = m.borrow().get(id) {
            let _ = MOUSE_FN.with(|f| {
                Reflect::apply(
                    f,
                    &JsValue::NULL,
                    &Array::of5(
                        chart,
                        &JsValue::from_str(event_type),
                        &JsValue::from_f64(x),
                        &JsValue::from_f64(y),
                        &styles,
                    ),
                )
            });
        }
    });
}

/// Worker-side render — the closure body shipped to `run_blocking`. The chart
/// `C` arrived by pointer; serialize here, pair with the transferred canvas,
/// build, and keep the instance for later updates.
pub fn render<C: ChartExt>(
    chart: C,
    id: String,
    plugins: String,
    defaults: String,
) -> Result<(), String> {
    let obj = chart.into_json(); // serialization happens on the worker
    let chart_js = CANVASES
        .with(|c| {
            c.borrow()
                .get(&id)
                .map(|entry| build_chart(entry, obj, &plugins, &defaults))
        })
        .ok_or_else(|| format!("chart-js-rs: no OffscreenCanvas transferred for `{id}`"))?
        .map_err(|e| format!("chart-js-rs: build failed for `{id}`: {e:?}"))?;
    CHARTS.with(|m| m.borrow_mut().insert(id, chart_js));
    Ok(())
}

/// Worker-side update — also handed to `run_blocking`. `false` if no live
/// instance exists for the id or the Chart.js update threw.
pub fn update<C: ChartExt>(chart: C, id: String, animate: bool) -> bool {
    let updated = chart.into_json();
    CHARTS.with(|m| match m.borrow().get(&id) {
        Some(chart_js) => update_chart(chart_js, updated, animate),
        None => false,
    })
}

/// Tear down a chart's worker-side state, calling Chart.js `destroy()` so the
/// canvas context is released rather than leaked.
pub fn forget_chart(id: &str) {
    CANVASES.with(|c| c.borrow_mut().remove(id));
    if let Some(chart) = CHARTS.with(|m| m.borrow_mut().remove(id)) {
        let _ = call_method(&chart, "destroy", &[]);
    }
}

/// One-time worker setup, driven by a `run` call right after the worker boots:
/// import Chart.js / luxon / adapter, register the canvas-background plugin
/// global, and install the message listener feeding `store_canvas`/`handle_mouse`.
pub async fn bootstrap(libs: WorkerLibs) -> Result<(), String> {
    let err = |c: &str, e: JsValue| format!("chart-js-rs bootstrap: {c}: {e:?}");

    dyn_import(&libs.chart_js)
        .await
        .map_err(|e| err("Chart.js import", e))?;

    // luxon: import the ESM build and bind its namespace to `self.luxon`. The
    // adapter reads the global `luxon`, and a UMD global build does not reliably
    // attach itself under `import()` (unlike a classic <script>) — but the ESM
    // build's import() *returns* the luxon namespace, so we bind that directly.
    let luxon = dyn_import(&libs.luxon)
        .await
        .map_err(|e| err("luxon import", e))?;
    Reflect::set(&js_sys::global(), &"luxon".into(), &luxon)
        .map_err(|e| err("bind self.luxon", e))?;

    dyn_import(&libs.luxon_adapter)
        .await
        .map_err(|e| err("luxon adapter import", e))?;

    // The convenience canvas-background plugin, on globalThis so an eval'd
    // `plugins` block can reference it. Shared with the main-thread path.
    crate::exports::register_chart_area_background();

    // `FnWithArgs::run_rust_fn` (and the tooltip `rationalise_2_levels` helpers)
    // emit bodies that call `window.callbacks.<fn>(...)`. On a worker there is
    // no `window`, so recreate a minimal one whose `callbacks` is the app's wasm
    // glue. worxide retains the glue it imported at `self.__worxide_glue` — that
    // is the contract these two crates share (see worxide's worker.js).
    {
        let g = js_sys::global();
        let glue = Reflect::get(&g, &"__worxide_glue".into())
            .map_err(|e| err("read __worxide_glue", e))?;
        if glue.is_undefined() || glue.is_null() {
            return Err(
                "chart-js-rs bootstrap: worxide glue not found on the worker \
                        (self.__worxide_glue is unset)"
                    .into(),
            );
        }
        let window = Reflect::get(&g, &"window".into())
            .ok()
            .filter(|w| w.is_object())
            .unwrap_or_else(|| Object::new().into());
        Reflect::set(&window, &"callbacks".into(), &glue)
            .map_err(|e| err("set window.callbacks", e))?;
        Reflect::set(&g, &"window".into(), &window).map_err(|e| err("set self.window", e))?;
    }

    install_message_listener();
    Ok(())
}

/// Additive `message` listener for our side-channel frames. Coexists with
/// worxide's own listener; each ignores the other's frames.
fn install_message_listener() {
    let cb = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |ev: web_sys::MessageEvent| {
        let data = ev.data();
        let get = |k: &str| Reflect::get(&data, &k.into()).unwrap_or(JsValue::UNDEFINED);
        match get("type").as_string().as_deref() {
            Some("cjsrs-canvas") => {
                store_canvas(
                    get("id").as_string().unwrap_or_default(),
                    get("canvas"),
                    get("width").as_f64().unwrap_or(0.0),
                    get("height").as_f64().unwrap_or(0.0),
                );
            }
            Some("cjsrs-mouse") => {
                handle_mouse(
                    &get("chartId").as_string().unwrap_or_default(),
                    &get("eventType").as_string().unwrap_or_default(),
                    get("x").as_f64().unwrap_or(0.0),
                    get("y").as_f64().unwrap_or(0.0),
                    get("computedStyles"),
                );
            }
            _ => {} // worxide frame or unrelated; not ours.
        }
    });
    let _ =
        worker_global().add_event_listener_with_callback("message", cb.as_ref().unchecked_ref());
    cb.forget(); // lives for the worker's lifetime
}

// ===========================================================================
// Main side
// ===========================================================================

/// A handle to a persistent chart worker. Cheap to clone (shares one worker).
#[derive(Clone)]
pub struct ChartWorker {
    worker: Rc<worxide::Worker>,
}

impl ChartWorker {
    /// Boot a worker and run its one-time bootstrap (default lib URLs).
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_libs(WorkerLibs::default()).await
    }

    /// As [`new`](Self::new), with custom JS dependency URLs.
    pub async fn with_libs(libs: WorkerLibs) -> Result<Self, Box<dyn std::error::Error>> {
        let worker = worxide::Worker::new().await.map_err(|e| e.to_string())?;
        worker
            .run(move || async move { bootstrap(libs).await })
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
        Ok(Self {
            worker: Rc::new(worker),
        })
    }

    /// Render a chart off the main thread: transfer the canvas, wire DOM mouse
    /// forwarding, then dispatch `render` with the chart moved across by pointer
    /// (no main-thread serialization).
    pub(crate) async fn render<C: ChartExt + Send + 'static>(
        &self,
        chart: C,
        id: String,
        plugins: String,
        defaults: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.transfer_canvas(&id)?;
        self.install_dom_mouse_handlers(&id)?;
        self.worker
            .run_blocking(move || render(chart, id, plugins, defaults))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
    }

    pub(crate) async fn update<C: ChartExt + Send + 'static>(
        &self,
        chart: C,
        id: String,
        animate: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self
            .worker
            .run_blocking(move || update(chart, id, animate))
            .await
            .map_err(|e| e.to_string())?)
    }

    /// Measure the `<canvas>`, hand its control to an OffscreenCanvas, and
    /// transfer that to the worker — the one place a transfer list is required,
    /// so it goes over `raw()` rather than worxide's dispatch.
    fn transfer_canvas(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let el = gloo_utils::document()
            .get_element_by_id(id)
            .ok_or_else(|| format!("no element with id `{id}`"))?;
        let width = el.client_width() as f64;
        let height = el.client_height() as f64;

        let offscreen = el
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| format!("element `{id}` is not a <canvas>"))?
            .transfer_control_to_offscreen()
            .map_err(|e| format!("{e:?}"))?;

        let msg = Object::new();
        let set = |k: &str, v: &JsValue| -> Result<(), Box<dyn std::error::Error>> {
            Reflect::set(&msg, &k.into(), v).map_err(|e| format!("{e:?}"))?;
            Ok(())
        };
        set("type", &"cjsrs-canvas".into())?;
        set("id", &id.into())?;
        set("canvas", &offscreen)?;
        set("width", &JsValue::from_f64(width))?;
        set("height", &JsValue::from_f64(height))?;

        self.worker
            .raw()
            .post_message_with_transfer(&msg, &Array::of1(&offscreen))
            .map_err(|e| format!("{e:?}"))?;
        Ok(())
    }

    /// Forward mouse events from the DOM canvas to the worker (it has no DOM, so
    /// we ship scaled coordinates + computed styles).
    fn install_dom_mouse_handlers(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let el = gloo_utils::document()
            .get_element_by_id(id)
            .ok_or_else(|| format!("no element with id `{id}`"))?;
        let width = el.client_width() as f64;
        let height = el.client_height() as f64;
        let rect = el.get_bounding_client_rect();
        let styles = web_sys::window()
            .and_then(|w| w.get_computed_style(&el).ok().flatten())
            .ok_or("could not read computed style")?;

        let computed = || {
            let o = Object::new();
            for (k, css) in [
                ("fontFamily", "font-family"),
                ("fontSize", "font-size"),
                ("fontWeight", "font-weight"),
                ("fontStyle", "font-style"),
                ("lineHeight", "line-height"),
                ("color", "color"),
            ] {
                let _ = Reflect::set(
                    &o,
                    &k.into(),
                    &styles.get_property_value(css).unwrap_or_default().into(),
                );
            }
            o
        };

        for (event_type, with_xy, with_styles) in [
            ("mousemove", true, true),
            ("mouseleave", false, false),
            ("click", true, true),
        ] {
            let worker = self.worker.clone();
            let chart_id = id.to_string();
            let rect = rect.clone();
            let computed = computed();
            let cb =
                Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                    let msg = Object::new();
                    let _ = Reflect::set(&msg, &"type".into(), &"cjsrs-mouse".into());
                    let _ = Reflect::set(&msg, &"eventType".into(), &event_type.into());
                    let _ = Reflect::set(&msg, &"chartId".into(), &chart_id.clone().into());
                    if with_xy {
                        let x = (e.client_x() as f64 - rect.left()) * (width / rect.width());
                        let y = (e.client_y() as f64 - rect.top()) * (height / rect.height());
                        let _ = Reflect::set(&msg, &"x".into(), &JsValue::from_f64(x));
                        let _ = Reflect::set(&msg, &"y".into(), &JsValue::from_f64(y));
                    }
                    if with_styles {
                        let _ = Reflect::set(&msg, &"computedStyles".into(), &computed);
                    }
                    let _ = worker.raw().post_message(&msg);
                });
            el.add_event_listener_with_callback(event_type, cb.as_ref().unchecked_ref())
                .map_err(|e| format!("{e:?}"))?;
            cb.forget();
        }
        Ok(())
    }
}
