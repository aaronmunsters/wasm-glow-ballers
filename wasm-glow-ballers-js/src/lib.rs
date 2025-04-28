use wasm_bindgen::prelude::*;
use wasm_glow_ballers_lib::{
    MappingItemType, mapping_exports_with_type, mapping_imports_with_type, mapping_with_type,
};

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum JsMappingItemType {
    Global,
    Function,
    Memory,
    Table,
}

impl From<JsMappingItemType> for MappingItemType {
    fn from(value: JsMappingItemType) -> Self {
        match value {
            JsMappingItemType::Global => MappingItemType::Global,
            JsMappingItemType::Function => MappingItemType::Function,
            JsMappingItemType::Memory => MappingItemType::Memory,
            JsMappingItemType::Table => MappingItemType::Table,
        }
    }
}

#[wasm_bindgen]
pub fn analyse(target: Vec<u8>, item_type: JsMappingItemType) -> Result<JsValue, JsError> {
    let map = mapping_with_type(&target, &item_type.into()).map_err(JsError::from)?;
    Ok(serde_wasm_bindgen::to_value(&map)?)
}

#[wasm_bindgen]
pub fn analyse_exports(target: Vec<u8>, item_type: JsMappingItemType) -> Result<JsValue, JsError> {
    let map = mapping_exports_with_type(&target, &item_type.into()).map_err(JsError::from)?;
    Ok(serde_wasm_bindgen::to_value(&map)?)
}

#[wasm_bindgen]
pub fn analyse_imports(target: Vec<u8>, item_type: JsMappingItemType) -> Result<JsValue, JsError> {
    let map = mapping_imports_with_type(&target, &item_type.into()).map_err(JsError::from)?;
    Ok(serde_wasm_bindgen::to_value(&map)?)
}
