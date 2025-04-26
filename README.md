# Wasm Glow Ballers

**Wasm Glow Ballers** is a tool that retrieves a mapping of all exports and/or imports from a WebAssembly module.
The output of the `analyse` function is a `Map` where keys are the names of exports/imports and values are their corresponding indices.
Note that the imports are prefixed with their module name as `module_name::import_name`.

## Example usage:

```bash
# Clone this repo
git clone https://github.com/aaronmunsters/wasm-glow-ballers
# `cd` into the rust-wasm library
cd wasm-glow-ballers/wasm-glow-ballers-js/
# Package a NodeJS library
wasm-pack build --target nodejs # [1]
# Use the NodeJS library to log the mapping where the target is the library itself
node -e 'console.log(require("./pkg").analyse(require("fs").readFileSync("../demo/demo.wasm")))' # [2]
```

Which then outputs:

```
Reading WASM file: ./demo.wasm
Global:
  Both: Map(3) { 'pi' => 2, 'env::imported_global' => 0, 'answer' => 1 }
  Exported: Map(2) { 'answer' => 1, 'pi' => 2 }
  Imported: Map(1) { 'env::imported_global' => 0 }
Memory:
  Both: Map(2) { 'env::imported_memory' => 0, 'mem' => 1 }
  Exported: Map(1) { 'mem' => 1 }
  Imported: Map(1) { 'env::imported_memory' => 0 }
Function:
  Both: Map(4) {
  'add' => 1,
  'subtract' => 2,
  'env::imported_func' => 0,
  'use_imports' => 3
}
  Exported: Map(3) { 'add' => 1, 'subtract' => 2, 'use_imports' => 3 }
  Imported: Map(1) { 'env::imported_func' => 0 }
Table:
  Both: Map(2) { 'tbl' => 1, 'env::imported_table' => 0 }
  Exported: Map(1) { 'tbl' => 1 }
  Imported: Map(1) { 'env::imported_table' => 0 }
```

- `[1]`: Tested using [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (version 0.13.0)
- `[2]`: Running on [NodeJS](https://nodejs.org/en) (version 22)
