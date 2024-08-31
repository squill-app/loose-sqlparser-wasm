use loose_sqlparser::{parse_with_options, Options, Statement};
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_sql(sql: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let options: Options = if options.is_undefined() || options.is_null() {
        Options::default()
    } else {
        from_value(options)
            .map_err(|err| JsValue::from_str(&format!("Invalid options: {}", err)))?
    };

    let vec_statement: Vec<Statement<'_>> = parse_with_options(sql, options).collect();
    match to_value(&vec_statement) {
        Ok(js_value) => Ok(js_value),
        Err(err) => Err(JsValue::from_str(&format!("Serialization error: {}", err))),
    }
}
