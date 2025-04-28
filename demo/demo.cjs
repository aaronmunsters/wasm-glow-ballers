const wasmGlowBallers = require("../wasm-glow-ballers-js/pkg");
const fs = require("fs");

const wasmFileName = "./demo.wasm";
console.log(`Reading WASM file: ${wasmFileName}`);
const wasmBinary = fs.readFileSync(wasmFileName);

const itemTypes = [
    { type: wasmGlowBallers.JsMappingItemType.Global, name: "Global" },
    { type: wasmGlowBallers.JsMappingItemType.Memory, name: "Memory" },
    { type: wasmGlowBallers.JsMappingItemType.Function, name: "Function" },
    { type: wasmGlowBallers.JsMappingItemType.Table, name: "Table" },
];

const analyses = [
    { name: "Both", func: wasmGlowBallers.analyse },
    { name: "Exported", func: wasmGlowBallers.analyse_exports },
    { name: "Imported", func: wasmGlowBallers.analyse_imports },
];

itemTypes.forEach((item) => {
    console.log(`${item.name}:`);
    analyses.forEach((analysis) => {
        try {
            const mapping = analysis.func(wasmBinary, item.type);
            console.log(`  ${analysis.name}:`, mapping);
        } catch (error) {
            console.error(`Error during ${analysis.name} | ${item.name}:`, error);
        }
    });
});
