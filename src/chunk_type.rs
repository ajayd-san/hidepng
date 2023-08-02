use crate::errors::{Error, self};
use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        let within_valid_range = self
            .bytes
            .iter()
            .all(|&byte| ((byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')));

        if !within_valid_range {
            return false;
        }

        true && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = errors::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let within_valid_range = value
            .iter()
            .all(|&byte| ((byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')));

        if !within_valid_range {
            return Err(Error::InvalidCharacterSet(String::from_utf8(value.to_vec()).unwrap()));
        }

        if value[2].is_ascii_lowercase() {
            return Err(Error::InvalidReservebit);
        }

        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4 {
            return Err(Error::InvalidSize {
                r#type: "str",
                size: s.len(),
            });
        }

        let within_valid_range = s
            .bytes()
            .all(|byte| ((byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')));

        if !within_valid_range {
            return Err(Error::InvalidCharacterSet(s.to_string()));
        }

        let bytes = s.as_bytes();
        let bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];

        Ok(ChunkType { bytes })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cstring = std::str::from_utf8(&self.bytes).unwrap();
        write!(f, "{}", cstring)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        let within_valid_range = "Rust"
            .bytes()
            .all(|byte| ((byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')));

        dbg!(within_valid_range);
        assert!(!chunk.is_valid());

        let within_valid_range = "Rust"
            .bytes()
            .all(|byte| ((byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')));

        dbg!(within_valid_range);
        let chunk = ChunkType::from_str("Ru1t");

        let within_valid_range = "Ru1t"
            .bytes()
            .all(|byte| ((byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')));
        dbg!(within_valid_range);
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
