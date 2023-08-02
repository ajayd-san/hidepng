#![allow(unused)]
use std::{
    error,
    fmt::Display,
    io::{self, BufReader, Read},
    str::Utf8Error,
    string::FromUtf8Error,
};

use crate::chunk_type::ChunkType;
use crate::errors;

#[derive(Debug)]
struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Chunk::calculate_crc(&chunk_type, &data);

        Chunk {
            length: data.len().try_into().unwrap(),
            chunk_type,
            chunk_data: data,
            crc,
        }
    }

    fn calculate_crc(chunk_type: &ChunkType, chunk_data: &Vec<u8>) -> u32 {
        let crc_input: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .copied()
            .chain(chunk_data.iter().copied())
            .collect();

        crc32fast::hash(&crc_input)
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        let res_string = String::from_utf8(self.chunk_data.clone())?;
        Ok(res_string)
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .into_iter()
            .chain(self.chunk_type().bytes().into_iter())
            .chain(self.data().iter().cloned())
            .chain(self.crc().to_be_bytes())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Box<dyn error::Error>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (length_bytes, rest) = value.split_at(4);
        let length = u32::from_be_bytes(length_bytes.try_into()?);

        let (chunk_type_bytes, rest) = rest.split_at(4);
        let chunk_type_bytes: [u8; 4] = chunk_type_bytes.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        let (data_bytes, crc_bytes) = rest.split_at(length as usize);
        let chunk_data = data_bytes.to_vec();
        let crc = u32::from_be_bytes(crc_bytes.try_into()?);

        // validate crc
        let calculated_crc = Self::calculate_crc(&chunk_type, &chunk_data);

        if crc != calculated_crc {
            return Err(Box::from(errors::Error::CrcMismatch));
        }

        Ok(Chunk {
            length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
