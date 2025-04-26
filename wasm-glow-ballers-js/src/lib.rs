use wasm_bindgen::prelude::*;
use wasm_glow_ballers_lib::{
    MappingItemType, mapping_exports_with_type, mapping_imports_with_type, mapping_with_type,
};

#[wasm_bindgen]
pub enum JsMappingItemType {
    Global,
    Function,
    Memory,
    Table,
}

fn convert_item_type(js_type: JsMappingItemType) -> MappingItemType {
    match js_type {
        JsMappingItemType::Global => MappingItemType::Global,
        JsMappingItemType::Function => MappingItemType::Function,
        JsMappingItemType::Memory => MappingItemType::Memory,
        JsMappingItemType::Table => MappingItemType::Table,
    }
}

#[wasm_bindgen]
pub fn analyse(target: Vec<u8>, item_type: JsMappingItemType) -> Result<JsValue, JsError> {
    let rust_item_type = convert_item_type(item_type);
    let map = mapping_with_type(&target, rust_item_type).map_err(JsError::from)?;
    Ok(serde_wasm_bindgen::to_value(&map)?)
}

#[wasm_bindgen]
pub fn analyse_exports(target: Vec<u8>, item_type: JsMappingItemType) -> Result<JsValue, JsError> {
    let rust_item_type = convert_item_type(item_type);
    let map = mapping_exports_with_type(&target, rust_item_type).map_err(JsError::from)?;
    Ok(serde_wasm_bindgen::to_value(&map)?)
}

#[wasm_bindgen]
pub fn analyse_imports(target: Vec<u8>, item_type: JsMappingItemType) -> Result<JsValue, JsError> {
    let rust_item_type = convert_item_type(item_type);
    let map = mapping_imports_with_type(&target, rust_item_type).map_err(JsError::from)?;
    Ok(serde_wasm_bindgen::to_value(&map)?)
}
