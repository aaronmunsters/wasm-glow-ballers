# Wasm Glow Ballers

**Wasm Glow Ballers** is a tool that retrieves a mapping of all global exports from a WebAssembly module.
The output is a `Map` where keys are the names of the exports and values are the indices of the exported globals.

## Example usage:

```bash
# Clone this repo
git clone https://github.com/aaronmunsters/wasm-glow-ballers
# `cd` into the rust-wasm library
cd wasm-glow-ballers/wasm-glow-ballers-js/
# Package a NodeJS library
wasm-pack build --target nodejs # [1]
# Use the NodeJS library to log the mapping where the target is the library itself
node -e 'console.log(require("./pkg").analyse(require("fs").readFileSync("./pkg/wasm_glow_ballers_js_bg.wasm")))' # [2]
```

Which then outputs `Map(0) {}`.

- `[1]`: Tested using [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (version 0.13.0)
- `[2]`: Running on [NodeJS](https://nodejs.org/en) (version 22)
