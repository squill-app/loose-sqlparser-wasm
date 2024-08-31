use loose_sqlparser::{parse, Statement};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

pub struct Token {
    pub value: String,
    pub token_type: String,
}

#[wasm_bindgen]
pub fn parse_sql(sql: &str) -> Result<JsValue, JsValue> {
    let vec_statement: Vec<Statement<'_>> = parse(sql).collect();
    match to_value(&vec_statement) {
        Ok(js_value) => Ok(js_value),
        Err(err) => Err(JsValue::from_str(&format!("Serialization error: {}", err))),
    }
}
