use wasm_bindgen::JsValue;

pub fn tick_callback(js: &str, arg1: &str, arg2: &str, arg3: &str) -> JsValue
{
    JsValue::from_str(&format!(
        r#"function ({arg1}, {arg2}, {arg3}) {{
    {js}
}}
"#,
    ))
}
pub fn single_arg_callback(js: &str, arg: &str) -> JsValue
{
    JsValue::from_str(&format!(
        r#"function ({arg}) {{
    {js}
}}
"#,
    ))
}
