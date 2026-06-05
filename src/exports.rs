//! Main-thread Chart.js operations, in Rust (formerly `js/exports.js`).
//!
//! `Chart::render` (see `utils.rs`) has already turned every `FnWithArgs` into a
//! real `Function` via `rationalise_js` before these run, so there is no
//! derationalize step here — just the Chart.js calls themselves, done through
//! `js-sys` (`Reflect`/`Function`) against the global `Chart`. DOM access goes
//! through `gloo-utils` (already a dependency), so this stays on stable Rust.

use js_sys::{Array, Function, Reflect};
use wasm_bindgen::{JsCast, JsValue};

/// The global `Chart` (set by the Chart.js UMD `<script>` on the page).
fn chart_global() -> JsValue {
    Reflect::get(&js_sys::global(), &"Chart".into()).unwrap_or(JsValue::UNDEFINED)
}

/// `Chart.getChart(document.getElementById(id))`, or `UNDEFINED` if missing.
fn live_chart(id: &str) -> Result<JsValue, JsValue> {
    let el = gloo_utils::document()
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str("element not found"))?;
    let chart_g = chart_global();
    let get_chart = Reflect::get(&chart_g, &"getChart".into())?.dyn_into::<Function>()?;
    get_chart.call1(&chart_g, &el.into())
}

/// Register the convenience `chartAreaBackground` plugin on `globalThis` so an
/// eval'd `plugins` block can reference it (idempotent). Shared with the worker
/// bootstrap.
pub(crate) fn register_chart_area_background() {
    let g = js_sys::global();
    let present = Reflect::get(&g, &"chartAreaBackground".into())
        .map(|v| !v.is_undefined())
        .unwrap_or(false);
    if present {
        return;
    }
    let _ = js_sys::eval(
        "globalThis.chartAreaBackground = { id: 'canvas_background_color', \
         beforeDraw: (chart, args, options) => { const { ctx, width, height } = chart; \
         ctx.save(); ctx.fillStyle = options.color || 'white'; ctx.fillRect(0, 0, width, height); \
         ctx.restore(); } };",
    );
}

/// Read back a rendered chart's config: `Chart.getChart(#id).config._config`.
/// Used by `ChartExt::get_chart_from_id`.
pub fn get_chart(id: &str) -> JsValue {
    let go = || -> Result<JsValue, JsValue> {
        let chart = live_chart(id)?;
        let config = Reflect::get(&chart, &"config".into())?;
        Reflect::get(&config, &"_config".into())
    };
    go().unwrap_or(JsValue::UNDEFINED)
}

/// Build a chart on the `#id` canvas: optional `defaults`/`plugins` eval, the
/// optional `window.mutate_chart_object` hook, then `new Chart(el, obj)`.
pub fn render_chart(v: JsValue, id: &str, mutate: bool, plugins: String, defaults: String) {
    register_chart_area_background();

    if !defaults.is_empty() {
        // Side-effecting block (e.g. `Chart.defaults.*`), in global scope.
        let _ = js_sys::eval(&defaults);
    }
    if !plugins.is_empty() {
        if let Ok(p) = js_sys::eval(&plugins) {
            let _ = Reflect::set(&v, &"plugins".into(), &p);
        }
    }

    // Optional main-thread mutate hook (`window.mutate_chart_object`).
    let obj = if mutate {
        let g = js_sys::global();
        let window = Reflect::get(&g, &"window".into()).unwrap_or_else(|_| g.into());
        match Reflect::get(&window, &"mutate_chart_object".into())
            .ok()
            .and_then(|f| f.dyn_into::<Function>().ok())
        {
            Some(f) => f.call1(&window, &v).unwrap_or(v),
            None => v,
        }
    } else {
        v
    };

    let Some(el) = gloo_utils::document().get_element_by_id(id) else {
        return;
    };
    if let Ok(ctor) = chart_global().dyn_into::<Function>() {
        // new Chart(el, obj)
        let _ = Reflect::construct(&ctor, &Array::of2(&el.into(), &obj));
    }
}

/// Swap a live chart's config and update it. Returns whether it succeeded.
pub fn update_chart(updated: JsValue, id: &str, animate: bool) -> bool {
    let go = || -> Result<(), JsValue> {
        let chart = live_chart(id)?;
        let inner = Reflect::get(&Reflect::get(&chart, &"config".into())?, &"_config".into())?;
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
        let update = Reflect::get(&chart, &"update".into())?.dyn_into::<Function>()?;
        if animate {
            update.call0(&chart)?;
        } else {
            update.call1(&chart, &JsValue::from_str("none"))?;
        }
        Ok(())
    };
    go().is_ok()
}
