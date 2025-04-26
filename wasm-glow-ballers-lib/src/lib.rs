pub mod error;

use std::collections::HashMap;

use error::Error;
use walrus::Module;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum MappingItemType {
    Global,
    Function,
    Memory,
    Table,
}

// // Creates a mapping of all exported and imported globals (original use case of the library)
// pub fn mapping(wasm: &[u8]) -> Result<HashMap<String, usize>, Error> {
//     mapping_with_type(wasm, MappingItemType::Global)
// }

// Creates a mapping of all exported and imported items of the specified type
pub fn mapping_with_type(
    wasm: &[u8],
    item_type: &MappingItemType,
) -> Result<HashMap<String, usize>, Error> {
    let module = Module::from_buffer(wasm).map_err(|_| Error::ParsingFailed)?;
    let mut result = HashMap::new();

    // Add exports
    let exports = mapping_exports_with_type_and_module(&module, &item_type)?;
    result.extend(exports);

    // Add imports
    let imports = mapping_imports_with_type_and_module(&module, &item_type)?;
    result.extend(imports);

    Ok(result)
}

// Creates a mapping of only exported items of the specified type from raw WASM bytes
pub fn mapping_exports_with_type(
    wasm: &[u8],
    item_type: &MappingItemType,
) -> Result<HashMap<String, usize>, Error> {
    let module = Module::from_buffer(wasm).map_err(|_| Error::ParsingFailed)?;
    mapping_exports_with_type_and_module(&module, &item_type)
}

// Creates a mapping of only imported items of the specified type from raw WASM bytes
pub fn mapping_imports_with_type(
    wasm: &[u8],
    item_type: &MappingItemType,
) -> Result<HashMap<String, usize>, Error> {
    let module = Module::from_buffer(wasm).map_err(|_| Error::ParsingFailed)?;
    mapping_imports_with_type_and_module(&module, &item_type)
}

// Creates a mapping of exported items of the specified type from a parsed Module
pub fn mapping_exports_with_type_and_module(
    module: &Module,
    item_type: &MappingItemType,
) -> Result<HashMap<String, usize>, Error> {
    let mut mapping = HashMap::new();

    for export in module.exports.iter() {
        match (item_type, &export.item) {
            (MappingItemType::Global, walrus::ExportItem::Global(id)) => {
                mapping.insert(export.name.to_string(), id.index());
            }
            (MappingItemType::Function, walrus::ExportItem::Function(id)) => {
                mapping.insert(export.name.to_string(), id.index());
            }
            (MappingItemType::Memory, walrus::ExportItem::Memory(id)) => {
                mapping.insert(export.name.to_string(), id.index());
            }
            (MappingItemType::Table, walrus::ExportItem::Table(id)) => {
                mapping.insert(export.name.to_string(), id.index());
            }
            _ => {}
        }
    }

    Ok(mapping)
}

// Creates a mapping of imported items of the specified type from a parsed Module
pub fn mapping_imports_with_type_and_module(
    module: &Module,
    item_type: &MappingItemType,
) -> Result<HashMap<String, usize>, Error> {
    let mut mapping = HashMap::new();

    for import in module.imports.iter() {
        let key = format!("{}::{}", import.module, import.name);

        match (item_type, &import.kind) {
            (MappingItemType::Global, walrus::ImportKind::Global(id)) => {
                mapping.insert(key, id.index());
            }
            (MappingItemType::Function, walrus::ImportKind::Function(id)) => {
                mapping.insert(key, id.index());
            }
            (MappingItemType::Memory, walrus::ImportKind::Memory(id)) => {
                mapping.insert(key, id.index());
            }
            (MappingItemType::Table, walrus::ImportKind::Table(id)) => {
                mapping.insert(key, id.index());
            }
            _ => {}
        }
    }

    Ok(mapping)
}
