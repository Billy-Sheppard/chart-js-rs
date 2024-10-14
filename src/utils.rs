use js_sys::{Array, Function, Object, Reflect};
use serde::{de, Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{exports::*, FnWithArgsOrAny};

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

fn rationalise_1_level<const N: usize>(obj: &JsValue, name: &'static str) {
    if let Ok(a) = Reflect::get(obj, &name.into()) {
        // If the property is undefined, dont try serialize it
        if a == JsValue::UNDEFINED {
            return;
        }

        if let Ok(o) = serde_wasm_bindgen::from_value::<FnWithArgsOrAny<N>>(a) {
            match o {
                FnWithArgsOrAny::Any(_) => (),
                FnWithArgsOrAny::FnWithArgs(fnwa) => {
                    let _ = Reflect::set(obj, &name.into(), &fnwa.build());
                }
            }
        }
    }
}
fn rationalise_2_levels<const N: usize>(obj: &JsValue, name: (&'static str, &'static str)) {
    if let Ok(a) = Reflect::get(obj, &name.0.into()) {
        // If the property is undefined, dont try serialize it
        if a == JsValue::UNDEFINED {
            return;
        }

        if let Ok(b) = Reflect::get(&a, &name.1.into()) {
            // If the property is undefined, dont try serialize it
            if b == JsValue::UNDEFINED {
                return;
            }

            if let Ok(o) = serde_wasm_bindgen::from_value::<FnWithArgsOrAny<N>>(b) {
                match o {
                    FnWithArgsOrAny::Any(_) => (),
                    FnWithArgsOrAny::FnWithArgs(fnwa) => {
                        let _ = Reflect::set(&a, &name.1.into(), &fnwa.build());
                    }
                }
            }
        }
    }
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
                rationalise_1_level::<2>(&dataset, "backgroundColor");
                rationalise_2_levels::<1>(&dataset, ("segment", "borderDash"));
                rationalise_2_levels::<1>(&dataset, ("segment", "borderColor"));
                rationalise_2_levels::<1>(&dataset, ("datalabels", "align"));
                rationalise_2_levels::<1>(&dataset, ("datalabels", "anchor"));
                rationalise_2_levels::<1>(&dataset, ("datalabels", "backgroundColor"));
                rationalise_2_levels::<2>(&dataset, ("datalabels", "formatter"));
                rationalise_2_levels::<1>(&dataset, ("datalabels", "offset"));
            });

        // Handle options.scales
        if let Some(scales) = object_values_at(&self.obj, "options.scales") {
            Object::values(&scales.dyn_into().unwrap())
                .iter()
                .for_each(|scale| {
                    rationalise_2_levels::<3>(&scale, ("ticks", "callback"));
                });
        }

        // Handle options.plugins.legend
        if let Some(legend) = object_values_at(&self.obj, "options.plugins.legend") {
            rationalise_2_levels::<2>(&legend, ("labels", "filter"));
        }
        // Handle options.plugins.tooltip
        if let Some(legend) = object_values_at(&self.obj, "options.plugins.tooltip") {
            rationalise_1_level::<1>(&legend, "filter");
            rationalise_2_levels::<1>(&legend, ("callbacks", "label"));
            rationalise_2_levels::<1>(&legend, ("callbacks", "title"));
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct JavascriptFunction {
    args: Vec<String>,
    body: String,
    return_value: String,
    closure_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FnWithArgs<const N: usize> {
    pub(crate) args: [String; N],
    pub(crate) body: String,
    pub(crate) return_value: String,
    pub(crate) closure_id: Option<String>,
}
const ALPHABET: [&str; 32] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "aa", "bb", "cc", "dd", "ee", "ff",
];
impl<const N: usize> Default for FnWithArgs<N> {
    fn default() -> Self {
        Self {
            args: (0..N)
                .map(|idx| ALPHABET[idx].to_string())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            body: Default::default(),
            return_value: Default::default(),
            closure_id: None,
        }
    }
}
impl<'de, const N: usize> Deserialize<'de> for FnWithArgs<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let js = JavascriptFunction::deserialize(deserializer)?;
        Ok(FnWithArgs::<N> {
            args: js.args.clone().try_into().map_err(|_| {
                de::Error::custom(format!("Array had length {}, needed {}.", js.args.len(), N))
            })?,
            body: js.body,
            return_value: js.return_value,
            closure_id: js.closure_id,
        })
    }
}
impl<const N: usize> Serialize for FnWithArgs<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        JavascriptFunction::serialize(
            &JavascriptFunction {
                args: self.args.to_vec(),
                body: self.body.clone(),
                return_value: self.return_value.clone(),
                closure_id: self.closure_id.clone(),
            },
            serializer,
        )
    }
}

impl<const N: usize> FnWithArgs<N> {
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn args<S: AsRef<str>>(mut self, args: [S; N]) -> Self {
        self.args = args
            .into_iter()
            .enumerate()
            .map(|(idx, s)| {
                let arg = s.as_ref();
                if arg.is_empty() { ALPHABET[idx] } else { arg }.to_string()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self
    }

    pub fn js_body(mut self, body: &str) -> Self {
        self.body = format!("{}\n{body}", self.body);
        self.to_owned()
    }

    pub fn js_return_value(mut self, return_value: &str) -> Self {
        self.return_value = return_value.to_string();
        self.to_owned()
    }

    pub fn build(self) -> Function {
        if let Some(id) = self.closure_id {
            let args = self.args.join(", ");
            Function::new_with_args(&args, &format!("{{ return window['{id}']({args}) }}"))
        } else {
            Function::new_with_args(
                &self.args.join(", "),
                &format!("{{ {}\nreturn {} }}", self.body, self.return_value),
            )
        }
    }
}

impl FnWithArgs<1> {
    pub fn run_rust_fn<A, B, FN: Fn(A) -> B>(mut self, _func: FN) -> Self {
        let fn_name = std::any::type_name::<FN>()
            .split("::")
            .collect::<Vec<_>>()
            .into_iter()
            .next_back()
            .unwrap();

        self.body = format!(
            "{}\nconst _out_ = window.callbacks.{}({});",
            self.body,
            fn_name,
            self.args.join(", ")
        );
        self.js_return_value("_out_")
    }

    #[track_caller]
    pub fn rust_closure<F: Fn(JsValue) -> JsValue + 'static>(mut self, closure: F) -> Self {
        let js_closure = wasm_bindgen::closure::Closure::wrap(
            Box::new(closure) as Box<dyn Fn(JsValue) -> JsValue>
        );
        let js_sys_fn: &js_sys::Function = js_closure.as_ref().unchecked_ref();

        let js_window = gloo_utils::window();
        let id = uuid::Uuid::new_v4().to_string();
        Reflect::set(&js_window, &JsValue::from_str(&id), js_sys_fn).unwrap();
        js_closure.forget();

        gloo_console::debug!(format!(
            "Closure at {}:{}:{} set at window.['{id}'].",
            file!(),
            line!(),
            column!()
        ));
        self.closure_id = Some(id);
        self
    }
}

impl FnWithArgs<2> {
    pub fn run_rust_fn<A, B, C, FN: Fn(A, B) -> C>(mut self, _func: FN) -> Self {
        let fn_name = std::any::type_name::<FN>()
            .split("::")
            .collect::<Vec<_>>()
            .into_iter()
            .next_back()
            .unwrap();

        self.body = format!(
            "{}\nconst _out_ = window.callbacks.{}({});",
            self.body,
            fn_name,
            self.args.join(", ")
        );
        self.js_return_value("_out_")
    }

    #[track_caller]
    pub fn rust_closure<F: Fn(JsValue, JsValue) -> JsValue + 'static>(
        mut self,
        closure: F,
    ) -> Self {
        let js_closure = wasm_bindgen::closure::Closure::wrap(
            Box::new(closure) as Box<dyn Fn(JsValue, JsValue) -> JsValue>
        );
        let js_sys_fn: &js_sys::Function = js_closure.as_ref().unchecked_ref();

        let js_window = gloo_utils::window();
        let id = uuid::Uuid::new_v4().to_string();
        Reflect::set(&js_window, &JsValue::from_str(&id), js_sys_fn).unwrap();
        js_closure.forget();

        gloo_console::debug!(format!(
            "Closure at {}:{}:{} set at window.['{id}'].",
            file!(),
            line!(),
            column!()
        ));
        self.closure_id = Some(id);
        self
    }
}

impl FnWithArgs<3> {
    pub fn run_rust_fn<A, B, C, D, FN: Fn(A, B, C) -> D>(mut self, _func: FN) -> Self {
        let fn_name = std::any::type_name::<FN>()
            .split("::")
            .collect::<Vec<_>>()
            .into_iter()
            .next_back()
            .unwrap();

        self.body = format!(
            "{}\nconst _out_ = window.callbacks.{}({});",
            self.body,
            fn_name,
            self.args.join(", ")
        );
        self.js_return_value("_out_")
    }

    #[track_caller]
    pub fn rust_closure<F: Fn(JsValue, JsValue, JsValue) -> JsValue + 'static>(
        mut self,
        closure: F,
    ) -> Self {
        let js_closure = wasm_bindgen::closure::Closure::wrap(
            Box::new(closure) as Box<dyn Fn(JsValue, JsValue, JsValue) -> JsValue>
        );
        let js_sys_fn: &js_sys::Function = js_closure.as_ref().unchecked_ref();

        let js_window = gloo_utils::window();
        let id = uuid::Uuid::new_v4().to_string();
        Reflect::set(&js_window, &JsValue::from_str(&id), js_sys_fn).unwrap();
        js_closure.forget();

        gloo_console::debug!(format!(
            "Closure at {}:{}:{} set at window.['{id}'].",
            file!(),
            line!(),
            column!()
        ));
        self.closure_id = Some(id);
        self
    }
}

impl FnWithArgs<4> {
    pub fn run_rust_fn<A, B, C, D, E, FN: Fn(A, B, C, D) -> E>(mut self, _func: FN) -> Self {
        let fn_name = std::any::type_name::<FN>()
            .split("::")
            .collect::<Vec<_>>()
            .into_iter()
            .next_back()
            .unwrap();

        self.body = format!(
            "{}\nconst _out_ = window.callbacks.{}({});",
            self.body,
            fn_name,
            self.args.join(", ")
        );
        self.js_return_value("_out_")
    }

    #[track_caller]
    pub fn rust_closure<F: Fn(JsValue, JsValue, JsValue, JsValue) -> JsValue + 'static>(
        mut self,
        closure: F,
    ) -> Self {
        let js_closure = wasm_bindgen::closure::Closure::wrap(
            Box::new(closure) as Box<dyn Fn(JsValue, JsValue, JsValue, JsValue) -> JsValue>
        );
        let js_sys_fn: &js_sys::Function = js_closure.as_ref().unchecked_ref();

        let js_window = gloo_utils::window();
        let id = uuid::Uuid::new_v4().to_string();
        Reflect::set(&js_window, &JsValue::from_str(&id), js_sys_fn).unwrap();
        js_closure.forget();

        gloo_console::debug!(format!(
            "Closure at {}:{}:{} set at window.['{id}'].",
            file!(),
            line!(),
            column!()
        ));
        self.closure_id = Some(id);
        self
    }
}

impl FnWithArgs<5> {
    pub fn run_rust_fn<A, B, C, D, E, F, FN: Fn(A, B, C, D, E) -> F>(mut self, _func: FN) -> Self {
        let fn_name = std::any::type_name::<FN>()
            .split("::")
            .collect::<Vec<_>>()
            .into_iter()
            .next_back()
            .unwrap();

        self.body = format!(
            "{}\nconst _out_ = window.callbacks.{}({});",
            self.body,
            fn_name,
            self.args.join(", ")
        );
        self.js_return_value("_out_")
    }

    #[track_caller]
    pub fn rust_closure<F: Fn(JsValue, JsValue, JsValue, JsValue, JsValue) -> JsValue + 'static>(
        mut self,
        closure: F,
    ) -> Self {
        let js_closure = wasm_bindgen::closure::Closure::wrap(Box::new(closure)
            as Box<dyn Fn(JsValue, JsValue, JsValue, JsValue, JsValue) -> JsValue>);
        let js_sys_fn: &js_sys::Function = js_closure.as_ref().unchecked_ref();

        let js_window = gloo_utils::window();
        let id = uuid::Uuid::new_v4().to_string();
        Reflect::set(&js_window, &JsValue::from_str(&id), js_sys_fn).unwrap();
        js_closure.forget();

        gloo_console::debug!(format!(
            "Closure at {}:{}:{} set at window.['{id}'].",
            file!(),
            line!(),
            column!()
        ));
        self.closure_id = Some(id);
        self
    }
}

impl FnWithArgs<6> {
    pub fn run_rust_fn<A, B, C, D, E, F, G, FN: Fn(A, B, C, D, E, F) -> G>(
        mut self,
        _func: FN,
    ) -> Self {
        let fn_name = std::any::type_name::<FN>()
            .split("::")
            .collect::<Vec<_>>()
            .into_iter()
            .next_back()
            .unwrap();

        self.body = format!(
            "{}\nconst _out_ = window.callbacks.{}({});",
            self.body,
            fn_name,
            self.args.join(", ")
        );
        self.js_return_value("_out_")
    }

    #[track_caller]
    pub fn rust_closure<
        F: Fn(JsValue, JsValue, JsValue, JsValue, JsValue, JsValue) -> JsValue + 'static,
    >(
        mut self,
        closure: F,
    ) -> Self {
        let js_closure = wasm_bindgen::closure::Closure::wrap(Box::new(closure)
            as Box<dyn Fn(JsValue, JsValue, JsValue, JsValue, JsValue, JsValue) -> JsValue>);
        let js_sys_fn: &js_sys::Function = js_closure.as_ref().unchecked_ref();

        let js_window = gloo_utils::window();
        let id = uuid::Uuid::new_v4().to_string();
        Reflect::set(&js_window, &JsValue::from_str(&id), js_sys_fn).unwrap();
        js_closure.forget();

        gloo_console::debug!(format!(
            "Closure at {}:{}:{} set at window.['{id}'].",
            file!(),
            line!(),
            column!()
        ));
        self.closure_id = Some(id);
        self
    }
}

// 7 is the maximum wasm_bindgen can handle rn AFAIK
impl FnWithArgs<7> {
    pub fn run_rust_fn<A, B, C, D, E, F, G, H, FN: Fn(A, B, C, D, E, F, G) -> H>(
        mut self,
        _func: FN,
    ) -> Self {
        let fn_name = std::any::type_name::<FN>()
            .split("::")
            .collect::<Vec<_>>()
            .into_iter()
            .next_back()
            .unwrap();

        self.body = format!(
            "{}\nconst _out_ = window.callbacks.{}({});",
            self.body,
            fn_name,
            self.args.join(", ")
        );
        self.js_return_value("_out_")
    }

    #[track_caller]
    pub fn rust_closure<
        F: Fn(JsValue, JsValue, JsValue, JsValue, JsValue, JsValue, JsValue) -> JsValue + 'static,
    >(
        mut self,
        closure: F,
    ) -> Self {
        let js_closure = wasm_bindgen::closure::Closure::wrap(Box::new(closure)
            as Box<
                dyn Fn(JsValue, JsValue, JsValue, JsValue, JsValue, JsValue, JsValue) -> JsValue,
            >);
        let js_sys_fn: &js_sys::Function = js_closure.as_ref().unchecked_ref();

        let js_window = gloo_utils::window();
        let id = uuid::Uuid::new_v4().to_string();
        Reflect::set(&js_window, &JsValue::from_str(&id), js_sys_fn).unwrap();
        js_closure.forget();

        gloo_console::debug!(format!(
            "Closure at {}:{}:{} set at window.['{id}'].",
            file!(),
            line!(),
            column!()
        ));
        self.closure_id = Some(id);
        self
    }
}
