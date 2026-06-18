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
//! building (`rationalise`, plugin/defaults `eval`, `new Chart`, config swap,
//! instance registry, listeners) is all Rust via `js-sys`. The two irreducible
//! bits are Rust-constructed `Function`s (no `.js` files):
//!   * a one-line dynamic-`import()` wrapper (`import` is syntax, not callable
//!     by reference), and
//!   * the Chart.js mouse/tooltip/legend interaction, which pokes Chart.js
//!     internals and is genuinely clearer as ~50 lines of JS than as `Reflect`.
//!
//! Everything else is Rust. `worker_shim.js` / `worker_imports.js` are gone.

use js_sys::{Array, Function, Object, Promise, Reflect};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

/// The default worker imports block: Chart.js, Luxon (ESM, bound to
/// `self.luxon`), and the Luxon date adapter.
///
/// The worker boot imports are just a block of JavaScript — the same model as
/// the pre-worxide `imports_block`. Pass your own to
/// [`ChartWorker::with_imports`] / [`WorkerChartExt::into_worker_chart`], or
/// extend this one: `format!("{DEFAULT_WORKER_IMPORTS}\n{my_extra_imports}")`.
/// Use dynamic `import(...)` (static `import` syntax is unavailable here) and
/// bind anything the adapter/plugins read as a global onto `self`.
pub const DEFAULT_WORKER_IMPORTS: &str = r#"
// Chart.js
await import("https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.js");

// Luxon (ESM): import() returns the module namespace; bind it to self.luxon so
// the date adapter (which reads the global `luxon`) finds it.
self.luxon = await import("https://cdn.jsdelivr.net/npm/luxon@^2/+esm");

// Luxon date adapter for Chart.js time scales.
await import("https://cdn.jsdelivr.net/npm/chartjs-adapter-luxon@^1/dist/chartjs-adapter-luxon.umd.min.js");
"#;

// ===========================================================================
// Worker side  — runs on the worker, via run/run_blocking closures or the
// message listener installed at bootstrap.
// ===========================================================================

struct CanvasEntry {
    canvas: JsValue,
    /// CSS pixels (the on-screen element's size).
    width: f64,
    height: f64,
    /// devicePixelRatio captured on the main thread. Applied as Chart.js's
    /// `options.devicePixelRatio`; Chart.js multiplies the bitmap by it once.
    dpr: f64,
}

thread_local! {
    /// OffscreenCanvases transferred in, keyed by chart id, awaiting `render`.
    static CANVASES: RefCell<HashMap<String, CanvasEntry>> = RefCell::new(HashMap::new());
    /// Live Chart.js instances, keyed by chart id.
    static CHARTS: RefCell<HashMap<String, JsValue>> = RefCell::new(HashMap::new());

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
        // Map DOM event names to the names Chart.js handles internally.
        const type = eventType === 'mouseleave' ? 'mouseout' : eventType;

        // A native-event stand-in. Chart.js reads x/y off the normalized event
        // (set below); native is only used for target / preventDefault.
        const native = {
            type, offsetX: x, offsetY: y, clientX: x, clientY: y,
            target: chart.canvas, currentTarget: chart.canvas,
            preventDefault() {}, stopPropagation() {},
        };
        // Normalized event shaped exactly like Chart.js's DOM platform builds.
        const ev = { type, chart, native, x, y };

        // Preferred path: route through Chart.js's real event handler — the same
        // code main-thread charts use. Hit-testing and hover/tooltip state match
        // the main thread, and legend-item clicks are handled natively by the
        // legend plugin. Note: Chart.js disables animations on an OffscreenCanvas
        // worker (issue #10305), so transitions are instant here regardless —
        // the tooltip updates correctly, just without the main-thread glide.
        if (typeof chart._eventHandler === 'function') {
            chart._eventHandler(ev);
            return;
        }

        // Fallback for a Chart.js build without _eventHandler: drive it manually.
        const mode = chart.options.interaction?.mode || 'nearest';
        const opts = chart.options.interaction || { intersect: false };
        if (type === 'mousemove') {
            chart.tooltip.setActiveElements(
                chart.getElementsAtEventForMode(ev, mode, opts, false), ev);
            chart.render();
        } else if (type === 'mouseout') {
            chart.tooltip.setActiveElements([], ev);
            chart.render();
        } else if (type === 'click') {
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
            if (chart.options.onClick) chart.options.onClick(ev, els, chart);
        }
        "#,
    );
}

/// Measure an element's CSS box and pin it to integer pixels.
///
/// The on-screen `<canvas>` is sized by CSS (e.g. `w-full`), which routinely
/// lands on fractional widths like `1255.5px`. The worker builds the
/// OffscreenCanvas bitmap from an integer size, so a fractional display box
/// forces the browser to resample the bitmap — the intermittent blur. Mirror
/// what Chart.js does on the main thread: floor the real (fractional) box to
/// integers and pin them as inline `width`/`height` styles, so the displayed
/// box is exactly the integer size we render at. Returns `(width, height)`.
fn pin_integer_size(el: &web_sys::Element) -> (f64, f64) {
    let rect = el.get_bounding_client_rect();
    let w = rect.width().floor().max(1.0);
    let h = rect.height().floor().max(1.0);
    if let Some(he) = el.dyn_ref::<web_sys::HtmlElement>() {
        let style = he.style();
        style.set_property("width", &format!("{w}px")).unwrap();
        style.set_property("height", &format!("{h}px")).unwrap();
    }
    (w, h)
}

/// Trailing-edge debounce: (re)arm a 100ms timer that fires `send_fn`.
fn debounce(timer: &Rc<Cell<i32>>, send_fn: &Function) {
    if let Some(w) = web_sys::window() {
        let t = timer.get();
        if t != 0 {
            w.clear_timeout_with_handle(t);
        }
        if let Ok(h) = w.set_timeout_with_callback_and_timeout_and_arguments_0(send_fn, 100) {
            timer.set(h);
        }
    }
}

/// DOM-side resize/zoom watchers for one worker chart. Dropping this (on chart
/// teardown) disconnects the ResizeObserver and removes the window listener.
pub(crate) struct ResizeWatchers {
    observer: Option<web_sys::ResizeObserver>,
    win_cb: Closure<dyn FnMut()>,
    _ro_cb: Closure<dyn FnMut()>,
    _send_cb: Closure<dyn FnMut()>,
}

impl Drop for ResizeWatchers {
    fn drop(&mut self) {
        if let Some(o) = &self.observer {
            o.disconnect();
        }
        if let Some(w) = web_sys::window() {
            w.remove_event_listener_with_callback("resize", self.win_cb.as_ref().unchecked_ref())
                .unwrap();
        }
    }
}

fn worker_global() -> web_sys::DedicatedWorkerGlobalScope {
    js_sys::global().unchecked_into()
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

/// Build a fresh Chart.js instance on the transferred canvas. `obj` is the
/// chart config already serialized (on this worker) by `into_json`.
fn build_chart(
    entry: &CanvasEntry,
    obj: JsValue,
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

    crate::utils::rationalise(&obj);

    // Crisp DPR, applied exactly once: set the bitmap to the CSS pixel size and
    // hand Chart.js `options.devicePixelRatio = dpr`. Chart.js's retinaScale
    // then multiplies the buffer to css*dpr and scales the context by dpr — the
    // single application. (Pre-multiplying the bitmap AND setting devicePixelRatio
    // double-counts and collapses the plot to a sliver.)
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

    // Force responsive off. There is no DOM element / ResizeObserver on a
    // worker, so Chart.js's responsive sizing resolves to a bogus size and
    // collapses the plot area. We drive sizing ourselves via the resize channel.
    let options = {
        let o = Reflect::get(&obj, &"options".into())?;
        if o.is_object() {
            o
        } else {
            let o: JsValue = Object::new().into();
            Reflect::set(&obj, &"options".into(), &o)?;
            o
        }
    };
    Reflect::set(&options, &"responsive".into(), &JsValue::FALSE)?;
    Reflect::set(&options, &"maintainAspectRatio".into(), &JsValue::FALSE)?;
    Reflect::set(
        &options,
        &"devicePixelRatio".into(),
        &JsValue::from_f64(entry.dpr),
    )?;

    let chart = Reflect::construct(&chart_ctor()?, &Array::of2(&entry.canvas, &obj))?;
    // Explicit logical resize so Chart.js sets the buffer to css*dpr.
    call_method(
        &chart,
        "resize",
        &[
            JsValue::from_f64(entry.width),
            JsValue::from_f64(entry.height),
        ],
    )
    .unwrap();

    // Initial animation unless options.animation === false.
    let animate = Reflect::get(&obj, &"options".into())
        .ok()
        .and_then(|o| Reflect::get(&o, &"animation".into()).ok())
        .map(|a| a != JsValue::FALSE)
        .unwrap_or(true);
    if animate {
        call_method(&chart, "update", &[JsValue::from_str("active")]).unwrap();
    }
    Ok(chart)
}

/// Swap a live chart's config and update. Returns whether it succeeded.
fn update_chart(chart: &JsValue, updated: JsValue, animate: bool) -> bool {
    let go = || -> Result<(), JsValue> {
        crate::utils::rationalise(&updated);
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
fn store_canvas(id: String, canvas: JsValue, width: f64, height: f64, dpr: f64) {
    CANVASES.with(|c| {
        c.borrow_mut().insert(
            id,
            CanvasEntry {
                canvas,
                width,
                height,
                dpr,
            },
        );
    });
}

/// Resize a live chart (container resize or zoom/DPR change). Updates the stored
/// size + dpr, applies the new `devicePixelRatio`, and calls Chart.js `resize`,
/// which re-multiplies the OffscreenCanvas bitmap to device px and redraws crisp.
fn resize_chart(id: &str, width: f64, height: f64, dpr: f64) {
    CANVASES.with(|c| {
        if let Some(e) = c.borrow_mut().get_mut(id) {
            e.width = width;
            e.height = height;
            e.dpr = dpr;
        }
    });
    CHARTS.with(|m| {
        if let Some(chart) = m.borrow().get(id) {
            if let Ok(opts) = Reflect::get(chart, &"options".into()) {
                Reflect::set(&opts, &"devicePixelRatio".into(), &JsValue::from_f64(dpr)).unwrap();
            }
            call_method(
                chart,
                "resize",
                &[JsValue::from_f64(width), JsValue::from_f64(height)],
            )
            .unwrap();
        }
    });
}

/// Route a forwarded mouse event to its chart's interaction handler.
fn handle_mouse(id: &str, event_type: &str, x: f64, y: f64, styles: JsValue) {
    CHARTS.with(|m| {
        if let Some(chart) = m.borrow().get(id) {
            MOUSE_FN
                .with(|f| {
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
                })
                .unwrap();
        }
    });
}

/// Worker-side render — the closure body shipped to `run_blocking`. The chart
/// `C` arrived by pointer; serialize here, pair with the transferred canvas,
/// build, and keep the instance for later updates.
pub fn render(
    chart: Box<dyn crate::WorkerChartExt>,
    id: String,
    plugins: String,
    defaults: String,
) -> Result<(), String> {
    let obj = chart.render_json(); // serialization happens on the worker
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
pub fn update(chart: Box<dyn crate::WorkerChartExt>, id: String, animate: bool) -> bool {
    let updated = chart.render_json();
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
        call_method(&chart, "destroy", &[]).unwrap();
    }
}

/// One-time worker setup, driven by a `run` call right after the worker boots:
/// run the user's imports block (Chart.js, date adapter, plugins), register the
/// canvas-background plugin
/// global, and install the message listener feeding `store_canvas`/`handle_mouse`.
pub async fn bootstrap(imports: String) -> Result<(), String> {
    let err = |c: &str, e: JsValue| format!("chart-js-rs bootstrap: {c}: {e:?}");

    // Run the user-controlled imports block (Chart.js, date adapter, plugins).
    // Wrapped in an async IIFE so `await import(...)` works; `import()` is
    // available in Function scope even though static `import` syntax is not.
    let run = Function::new_no_args(&format!("return (async () => {{\n{imports}\n}})();"));
    let promise = run
        .call0(&JsValue::NULL)
        .map_err(|e| err("imports block", e))?;
    JsFuture::from(Promise::from(promise))
        .await
        .map_err(|e| err("imports block", e))?;

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
                    {
                        let d = get("dpr").as_f64().unwrap_or(1.0);
                        if d > 0.0 {
                            d
                        } else {
                            1.0
                        }
                    },
                );
            }
            Some("cjsrs-resize") => {
                resize_chart(
                    &get("id").as_string().unwrap_or_default(),
                    get("width").as_f64().unwrap_or(0.0),
                    get("height").as_f64().unwrap_or(0.0),
                    {
                        let d = get("dpr").as_f64().unwrap_or(1.0);
                        if d > 0.0 {
                            d
                        } else {
                            1.0
                        }
                    },
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
        Self::with_imports(DEFAULT_WORKER_IMPORTS.to_string()).await
    }

    /// As [`new`](Self::new), with custom JS dependency URLs.
    pub async fn with_imports(imports: String) -> Result<Self, Box<dyn std::error::Error>> {
        let worker = worxide::Worker::new().await.map_err(|e| e.to_string())?;
        worker
            .run(move || async move { bootstrap(imports).await })
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
        Ok(Self {
            worker: Rc::new(worker),
        })
    }

    /// Run a consumer-provided setup closure ON THE WORKER, after `bootstrap`
    /// (Chart.js + plugin libs are loaded) and before the chart is built. This
    /// is the hook for registering custom Chart.js plugins on the worker's
    /// `Chart` global and for moving owned Rust state (captured in the closure)
    /// across to the worker. Generic: it runs whatever closure it's given.
    pub(crate) async fn run_setup(
        &self,
        f: Box<dyn FnOnce() + Send + 'static>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.worker
            .run(move || async move {
                f();
                Ok::<(), String>(())
            })
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
    }

    /// Terminate the underlying worker, freeing the OS worker thread and
    /// everything it held (OffscreenCanvas, Chart.js instance, worker-side
    /// state). Internal: the chart lifetime is managed for the consumer.
    pub(crate) fn terminate(&self) {
        self.worker.terminate();
    }

    /// Render a chart off the main thread: transfer the canvas, wire DOM mouse
    /// forwarding, then dispatch `render` with the chart moved across by pointer
    /// (no main-thread serialization).
    pub(crate) async fn render(
        &self,
        chart: Box<dyn crate::WorkerChartExt>,
        id: String,
        plugins: String,
        defaults: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Attach DOM mouse listeners BEFORE transferring control to the
        // OffscreenCanvas — matching the original worker. Listeners added after
        // transfer don't reliably fire, so order matters here.
        self.install_dom_mouse_handlers(&id)?;
        self.transfer_canvas(&id)?;
        self.worker
            .run_blocking(move || render(chart, id, plugins, defaults))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
    }

    pub(crate) async fn update(
        &self,
        chart: Box<dyn crate::WorkerChartExt>,
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
    /// Watch the on-screen canvas for size changes (ResizeObserver) and the
    /// window for zoom / DPR changes (resize event), posting debounced (100ms)
    /// `cjsrs-resize` frames so the worker resizes the chart crisply. The
    /// returned handle must be kept alive for the chart's lifetime; dropping it
    /// tears the watchers down.
    pub(crate) fn install_resize_watchers(
        &self,
        el: &web_sys::Element,
        id: &str,
    ) -> ResizeWatchers {
        let worker = self.worker.clone();
        let id = id.to_string();
        let el = el.clone();
        let timer: Rc<Cell<i32>> = Rc::new(Cell::new(0));

        // Trailing-edge sender: read current size + dpr, post one resize frame.
        let send_cb = Closure::<dyn FnMut()>::new({
            let worker = worker.clone();
            let id = id.clone();
            let el = el.clone();
            let timer = timer.clone();
            move || {
                timer.set(0);
                let (w, h) = pin_integer_size(&el);
                let dpr = web_sys::window()
                    .map(|x| x.device_pixel_ratio())
                    .filter(|d| *d > 0.0)
                    .unwrap_or(1.0);
                let msg = Object::new();
                Reflect::set(&msg, &"type".into(), &"cjsrs-resize".into()).unwrap();
                Reflect::set(&msg, &"id".into(), &id.clone().into()).unwrap();
                Reflect::set(&msg, &"width".into(), &JsValue::from_f64(w)).unwrap();
                Reflect::set(&msg, &"height".into(), &JsValue::from_f64(h)).unwrap();
                Reflect::set(&msg, &"dpr".into(), &JsValue::from_f64(dpr)).unwrap();
                worker.worker_handle().post_message(&msg).unwrap();
            }
        });
        let send_fn: Function = send_cb.as_ref().unchecked_ref::<Function>().clone();

        // ResizeObserver on the element (container / layout changes).
        let ro_cb = Closure::<dyn FnMut()>::new({
            let timer = timer.clone();
            let send_fn = send_fn.clone();
            move || debounce(&timer, &send_fn)
        });
        let observer = web_sys::ResizeObserver::new(ro_cb.as_ref().unchecked_ref()).ok();
        if let Some(o) = &observer {
            o.observe(&el);
        }

        // Window resize (zoom / viewport changes, incl. devicePixelRatio).
        let win_cb = Closure::<dyn FnMut()>::new({
            let timer = timer.clone();
            let send_fn = send_fn.clone();
            move || debounce(&timer, &send_fn)
        });
        if let Some(w) = web_sys::window() {
            w.add_event_listener_with_callback("resize", win_cb.as_ref().unchecked_ref())
                .unwrap();
        }

        ResizeWatchers {
            observer,
            win_cb,
            _ro_cb: ro_cb,
            _send_cb: send_cb,
        }
    }

    fn transfer_canvas(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let el = gloo_utils::document()
            .get_element_by_id(id)
            .ok_or_else(|| format!("no element with id `{id}`"))?;
        let (width, height) = pin_integer_size(&el);
        let dpr = web_sys::window()
            .map(|w| w.device_pixel_ratio())
            .filter(|d| *d > 0.0)
            .unwrap_or(1.0);

        let canvas = el
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| format!("element `{id}` is not a <canvas>"))?;

        // A canvas can be transferred exactly once, ever. Mark it on first
        // transfer and refuse a second attempt with a clean error instead of
        // letting `transferControlToOffscreen` throw `InvalidStateError`.
        if canvas.has_attribute("data-cjsrs-transferred") {
            return Err(format!(
                "canvas `{id}` was already transferred to a worker; render once \
                 per element (reuse goes through update)"
            )
            .into());
        }
        canvas.set_attribute("data-cjsrs-transferred", "1").unwrap();

        let offscreen = canvas
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
        set("dpr", &JsValue::from_f64(dpr))?;

        self.worker
            .worker_handle()
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
                Reflect::set(
                    &o,
                    &k.into(),
                    &styles.get_property_value(css).unwrap_or_default().into(),
                )
                .unwrap();
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
            let el_evt = el.clone();
            let computed = computed();
            let cb =
                Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                    let msg = Object::new();
                    Reflect::set(&msg, &"type".into(), &"cjsrs-mouse".into()).unwrap();
                    Reflect::set(&msg, &"eventType".into(), &event_type.into()).unwrap();
                    Reflect::set(&msg, &"chartId".into(), &chart_id.clone().into()).unwrap();
                    if with_xy {
                        // Read geometry fresh each event so coords stay correct
                        // after resize / zoom / scroll.
                        let rect = el_evt.get_bounding_client_rect();
                        let cw = el_evt.client_width() as f64;
                        let ch = el_evt.client_height() as f64;
                        let x = (e.client_x() as f64 - rect.left()) * (cw / rect.width());
                        let y = (e.client_y() as f64 - rect.top()) * (ch / rect.height());
                        Reflect::set(&msg, &"x".into(), &JsValue::from_f64(x)).unwrap();
                        Reflect::set(&msg, &"y".into(), &JsValue::from_f64(y)).unwrap();
                    }
                    if with_styles {
                        Reflect::set(&msg, &"computedStyles".into(), &computed).unwrap();
                    }
                    worker.worker_handle().post_message(&msg).unwrap();
                });
            el.add_event_listener_with_callback(event_type, cb.as_ref().unchecked_ref())
                .map_err(|e| format!("{e:?}"))?;
            cb.forget();
        }
        Ok(())
    }
}
