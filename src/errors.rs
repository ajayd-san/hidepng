use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Expected type {r#type:?} of size 4, got size {size:?}.")]
    InvalidSize { r#type: &'static str, size: usize },

    #[error("Input too small.")]
    InputTooSmall,

    #[error("Chunk type `{0}` not within valid ASCII set.")]
    InvalidCharacterSet(String),

    #[error("Reserve bit in chunk type `{0}` should be 0.")]
    InvalidReservebit(String),

    #[error("CRC check failed, possible corruption.")]
    CrcMismatch,

    #[error("Chunk Type `{0}` does not exist.")]
    ChunkTypeNotFound(String),

    #[error("{0}")]
    InvalidFile(&'static str),

    #[error("File Signature does not match standard PNG signature. Possible corruption.")]
    InvalidPngSignature,
}
