use js_sys::{Array, Function, Object, Reflect};
use serde::{de, Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{exports::*, FnWithArgsOrAny};

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
                rationalise_2_levels::<1>(&dataset, ("datalabels", "formatter"));
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
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FnWithArgs<const N: usize> {
    pub(crate) args: [String; N],
    pub(crate) body: String,
    pub(crate) return_value: String,
}
impl<const N: usize> Default for FnWithArgs<N> {
    fn default() -> Self {
        Self {
            args: [String::new()].into_iter().cycle().take(N).collect::<Vec<_>>().try_into().unwrap(),
            body: Default::default(),
            return_value: Default::default(),
        }
    }
}
impl<'de, const N: usize> Deserialize<'de> for FnWithArgs<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let js = JavascriptFunction::deserialize(deserializer)?;
        Ok(
            FnWithArgs::<N> {
                args: js.args.clone().try_into().map_err(|_| de::Error::custom(format!("Array had length {}, needed {}.", js.args.len(), N)))?,
                body: js.body,
                return_value: js.return_value,
            }
        )
    }
}
impl<const N: usize> Serialize for FnWithArgs<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        JavascriptFunction::serialize(&JavascriptFunction {
            args: self.args.to_vec(),
            body: self.body.clone(),
            return_value: self.return_value.clone(),
        }, serializer)
    }
}

impl<const N: usize> FnWithArgs<N> {
    pub fn is_empty(&self) -> bool {
        self.args.is_empty() && self.body.is_empty()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn args<S: AsRef<str>>(&mut self, args: [S; N]) -> &mut Self {
        self.args = args.map(|s| s.as_ref().to_string());
        self
    }

    pub fn run_rust_fn<F>(&mut self, in_vars: &[String], out_var: String, _: F) -> Self {
        self.body = format!(
            "{}\nconst {out_var} = window.callbacks.{}({});",
            self.body,
            std::any::type_name::<F>()
                .split("::")
                .collect::<Vec<_>>()
                .into_iter()
                .next_back()
                .unwrap(),
            in_vars.join(", ")
        );
        self.to_owned()
    }

    pub fn body(&mut self, body: &str) -> Self {
        self.body = format!("{}\n{body}", self.body);
        self.to_owned()
    }

    pub fn return_value(&mut self, return_value: &str) -> Self {
        self.return_value = return_value.to_string();
        self.to_owned()
    }

    pub fn build(&self) -> Function {
        Function::new_with_args(
            &self.args.join(", "),
            &format!("{{ {}\nreturn {} }}", self.body, self.return_value),
        )
    }
}
