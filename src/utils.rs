use js_sys::{Array, Object, Reflect};
use std::cell::RefCell;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{exports::*, BoolString, FnWithArgs, FnWithArgsOrT, NumberString};

pub fn get_order_fn(
    lhs: &crate::NumberOrDateString,
    rhs: &crate::NumberOrDateString,
) -> std::cmp::Ordering {
    crate::utils::ORDER_FN.with_borrow(|f| f(lhs, rhs))
}
pub fn set_order_fn<
    F: Fn(&crate::NumberOrDateString, &crate::NumberOrDateString) -> std::cmp::Ordering + 'static,
>(
    f: F,
) {
    let _ = ORDER_FN.replace(Box::new(f));
}

thread_local! {
    #[allow(clippy::type_complexity)]
    pub static ORDER_FN: RefCell<
        Box<dyn Fn(&crate::NumberOrDateString, &crate::NumberOrDateString) -> std::cmp::Ordering>,
    > = RefCell::new({
        Box::new(
            |lhs: &crate::NumberOrDateString, rhs: &crate::NumberOrDateString| -> std::cmp::Ordering {
                lhs.cmp(rhs)
            },
        )as Box<_>
    });
}

pub fn uncircle_chartjs_value_to_serde_json_value(
    js: impl AsRef<JsValue>,
) -> Result<serde_json::Value, String> {
    // this makes sure we don't get any circular objects, `JsValue` allows this, `serde_json::Value` does not!
    let blacklist_function =
        js_sys::Function::new_with_args("key, val", "if (!key.startsWith('$')) { return val; }");
    let js_string =
        js_sys::JSON::stringify_with_replacer(js.as_ref(), &JsValue::from(blacklist_function))
            .map_err(|e| e.as_string().unwrap_or_default())?
            .as_string()
            .unwrap();

    serde_json::from_str(&js_string).map_err(|e| e.to_string())
}

#[wasm_bindgen]
#[derive(Clone)]
#[must_use = "\nAppend .render()\n"]
pub struct Chart {
    pub(crate) obj: JsValue,
    pub(crate) id: String,
    pub(crate) mutate: bool,
    pub(crate) plugins: String,
    pub(crate) defaults: String,
}

/// Walks the JsValue object to get the value of a nested property
/// using the JS dot notation
fn get_path(j: &JsValue, item: &str) -> Option<JsValue> {
    let mut path = item.split('.');
    let item = &path.next().unwrap().to_string().into();
    let k = Reflect::get(j, item);

    if k.is_err() {
        return None;
    }

    let k = k.unwrap();
    if path.clone().count() > 0 {
        return get_path(&k, path.collect::<Vec<&str>>().join(".").as_str());
    }

    Some(k)
}

/// Get values of an object as an array at the given path.
/// See get_path()
fn object_values_at(j: &JsValue, item: &str) -> Option<JsValue> {
    let o = get_path(j, item);
    o.and_then(|o| {
        if o == JsValue::UNDEFINED {
            None
        } else {
            Some(o)
        }
    })
}

impl Chart {
    // pub fn new(chart: JsValue, id: String) -> Option<Self> {
    //     chart.is_object().then_some(Self{
    //         obj: chart,
    //         id,
    //         mutate: false,
    //         plugins: String::new(),
    //     })
    // }

    #[must_use = "\nAppend .render()\n"]
    pub fn mutate(&mut self) -> Self {
        self.mutate = true;
        self.clone()
    }

    #[must_use = "\nAppend .render()\n"]
    pub fn plugins(&mut self, plugins: impl Into<String>) -> Self {
        self.plugins = plugins.into();
        self.clone()
    }

    #[must_use = "\nAppend .render()\n"]
    pub fn defaults(&mut self, defaults: impl Into<String>) -> Self {
        self.defaults = format!("{}\n{}", self.defaults, defaults.into());
        self.to_owned()
    }

    pub fn render(self) {
        self.rationalise_js();
        render_chart(self.obj, &self.id, self.mutate, self.plugins, self.defaults);
    }

    pub fn update(self, animate: bool) -> bool {
        self.rationalise_js();
        update_chart(self.obj, &self.id, animate)
    }

    /// Converts serialized FnWithArgs to JS Function's
    /// For new chart options, this will need to be updated
    pub fn rationalise_js(&self) {
        // Handle data.datasets
        Array::from(&get_path(&self.obj, "data.datasets").unwrap())
            .iter()
            .for_each(|dataset| {
                FnWithArgsOrT::<2, String>::rationalise_1_level(&dataset, "backgroundColor");
                FnWithArgs::<1>::rationalise_2_levels(&dataset, ("segment", "borderDash"));
                FnWithArgs::<1>::rationalise_2_levels(&dataset, ("segment", "borderColor"));
                FnWithArgsOrT::<1, String>::rationalise_2_levels(&dataset, ("datalabels", "align"));
                FnWithArgsOrT::<1, String>::rationalise_2_levels(
                    &dataset,
                    ("datalabels", "anchor"),
                );
                FnWithArgsOrT::<1, String>::rationalise_2_levels(
                    &dataset,
                    ("datalabels", "backgroundColor"),
                );
                FnWithArgs::<2>::rationalise_2_levels(&dataset, ("datalabels", "formatter"));
                FnWithArgsOrT::<1, NumberString>::rationalise_2_levels(
                    &dataset,
                    ("datalabels", "offset"),
                );
                FnWithArgsOrT::<1, BoolString>::rationalise_2_levels(
                    &dataset,
                    ("datalabels", "display"),
                );
            });

        // Handle options.scales
        if let Some(scales) = object_values_at(&self.obj, "options.scales") {
            Object::values(&scales.dyn_into().unwrap())
                .iter()
                .for_each(|scale| {
                    FnWithArgs::<3>::rationalise_2_levels(&scale, ("ticks", "callback"));
                });
        }

        // Handle options.plugins.legend
        if let Some(legend) = object_values_at(&self.obj, "options.plugins.legend") {
            FnWithArgs::<2>::rationalise_2_levels(&legend, ("labels", "filter"));
            FnWithArgs::<3>::rationalise_2_levels(&legend, ("labels", "sort"));
        }
        // Handle options.plugins.tooltip
        if let Some(legend) = object_values_at(&self.obj, "options.plugins.tooltip") {
            FnWithArgs::<1>::rationalise_1_level(&legend, "filter");
            FnWithArgs::<1>::rationalise_2_levels(&legend, ("callbacks", "label"));
            FnWithArgs::<1>::rationalise_2_levels(&legend, ("callbacks", "title"));
        }
    }
}
