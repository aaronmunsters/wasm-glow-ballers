use std::collections::HashMap;

use walrus::Module;

pub fn mapping(wasm: &[u8]) -> HashMap<String, usize> {
    let module = Module::from_buffer(wasm).unwrap();
    let mut mapping = HashMap::new();
    for export in module.exports.iter() {
        if let walrus::ExportItem::Global(id) = export.item {
            mapping.insert(export.name.to_string(), id.index());
        }
    }
    return mapping;
}
