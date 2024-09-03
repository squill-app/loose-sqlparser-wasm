use loose_sqlparser::{parse_with_options, Options, Statement};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r#"
export type TokenType =
  | "Any"
  | "Comment"
  | "QuotedIdentifierOrConstant"
  | "NumericConstant"
  | "IdentifierOrKeyword"
  | "Operator"
  | "StatementDelimiter"
  | "Fragment";

export interface Token {
  value: string | Array<Token>;
  type: TokenType;
  start: {
    offset: number;
    line: number;
    column: number;
  };
  end: {
    offset: number;
    line: number;
    column: number;
  };
}

export interface Statement {
  input: string;
  tokens: Array<Token>;
}
"#;

#[wasm_bindgen(js_name = Options)]
pub struct JsOptions(Options);

#[allow(clippy::new_without_default)]
#[wasm_bindgen(js_class = Options)]
impl JsOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        JsOptions(Options::default())
    }

    #[wasm_bindgen(getter, js_name = statementDelimiter)]
    pub fn statement_delimiter(&self) -> String {
        self.0.statement_delimiter.clone()
    }

    #[wasm_bindgen(setter, js_name = statementDelimiter)]
    pub fn set_statement_delimiter(&mut self, value: String) {
        self.0.statement_delimiter = value;
    }
}

/// Parses the given SQL string using the provided options.
///
/// @param {string} sql - The SQL string to parse.
/// @param {Options | undefined} [options] - Optional parsing options.
/// @returns {Array<Statement>} - The parsed SQL statements.
#[wasm_bindgen(js_name = parseSql, skip_jsdoc)]
pub fn parse_sql(sql: &str, options: Option<JsOptions>) -> Result<JsValue, JsValue> {
    let options: Options = match options {
        None => Options::default(),
        Some(js_options) => js_options.0,
    };

    let vec_statement: Vec<Statement<'_>> = parse_with_options(sql, options).collect();
    match to_value(&vec_statement) {
        Ok(js_value) => Ok(js_value),
        Err(err) => Err(JsValue::from_str(&format!("Serialization error: {}", err))),
    }
}
