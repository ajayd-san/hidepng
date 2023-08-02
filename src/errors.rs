use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Expected type {r#type:?} of size 4, got size {size:?}.")]
    InvalidSize { r#type: &'static str, size: usize },

    #[error("Chunk type {0} not within valid ASCII set.")]
    InvalidCharacterSet(String),

    #[error("Reserve bit should be 0.")]
    InvalidReservebit,

    #[error("CRC check failed, possible corruption.")]
    CrcMismatch


}
