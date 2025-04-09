use wasm_bindgen::prelude::*;
use wasm_glow_ballers_lib::mapping;

#[wasm_bindgen]
pub fn analyse(target: Vec<u8>) -> Result<JsValue, JsError> {
    let map = mapping(&target).map_err(JsError::from)?;
    Ok(serde_wasm_bindgen::to_value(&map)?)
}
