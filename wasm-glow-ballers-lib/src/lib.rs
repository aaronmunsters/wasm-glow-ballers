pub mod error;

use std::collections::HashMap;

use error::Error;
use walrus::Module;

pub fn mapping(wasm: &[u8]) -> Result<HashMap<String, usize>, Error> {
    let module = Module::from_buffer(wasm).map_err(|_| Error::ParsingFailed)?;
    let mut mapping = HashMap::new();

    // Add exported globals
    for export in module.exports.iter() {
        if let walrus::ExportItem::Global(id) = export.item {
            mapping.insert(export.name.to_string(), id.index());
        }
    }

    // Add imported globals
    for import in module.imports.iter() {
        if let walrus::ImportKind::Global(id) = import.kind {
            mapping.insert(
                import.module.to_string() + "::" + &import.name.to_string(),
                id.index(),
            );
        }
    }

    return Ok(mapping);
}
