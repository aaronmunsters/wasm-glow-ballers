use wasm_bindgen::prelude::*;
use wasm_glow_ballers_lib::mapping;

#[wasm_bindgen]
pub fn analyse(target: Vec<u8>) -> JsValue {
    serde_wasm_bindgen::to_value(&mapping(&target)).unwrap()
}
