pub mod emojis;

#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

lazy_static! {
  static ref EMOJIS: BTreeMap<&'static str, &'static str> = {
    emojis::get_map()
  };
}

#[wasm_bindgen]
pub fn search_emoji(name: &str) -> JsValue {
  let mut results: Vec<&'static str> = Vec::new();
  for (_, v) in EMOJIS.range(name.to_lowercase().as_str()..) {
    results.push(v);
  }

  JsValue::from_serde(&results).unwrap()
}
