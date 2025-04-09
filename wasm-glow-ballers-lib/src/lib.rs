pub mod error;

use std::collections::HashMap;

use error::Error;
use walrus::Module;

pub fn mapping(wasm: &[u8]) -> Result<HashMap<String, usize>, Error> {
    let module = Module::from_buffer(wasm).map_err(|_| Error::ParsingFailed)?;
    let mut mapping = HashMap::new();
    for export in module.exports.iter() {
        if let walrus::ExportItem::Global(id) = export.item {
            mapping.insert(export.name.to_string(), id.index());
        }
    }
    return Ok(mapping);
}
