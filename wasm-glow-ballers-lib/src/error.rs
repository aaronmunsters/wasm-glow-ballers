use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Parsing the Wasm module failed")]
    ParsingFailed,
}
