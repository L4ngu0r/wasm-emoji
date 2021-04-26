#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

// TODO generate once & for all (cf emojis.js)
lazy_static! {
    static ref EMOJIS: BTreeMap<&'static str, &'static str> = {
        let mut map = BTreeMap::new();
        map.insert("grinning_face", "1F600");
        map.insert("grinning_face_with_big_eyes", "1F603");
        map.insert("grinning_face_with_smiling_eyes", "1F604");
        map
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
