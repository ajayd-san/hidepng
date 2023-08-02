use std::{fs, path::PathBuf, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, errors::Error, png::Png};

pub fn encode(
    file_path: PathBuf,
    chunk_type: ChunkType,
    msg: String,
    output_file: Option<PathBuf>,
) -> anyhow::Result<()> {
    let contents = fs::read(&file_path)?;
    let msg = msg.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, msg);

    let mut png = Png::try_from(contents.as_slice())?;
    png.append_chunk(chunk);

    let output = png.as_bytes();

    fs::write(output_file.unwrap_or(file_path), output)?;
    Ok(())
}

pub fn decode(file_path: PathBuf, chunk_type: ChunkType) -> anyhow::Result<String> {
    let contents = fs::read(file_path)?;
    let png = Png::try_from(contents.as_slice())?;

    let chunk = png
        .chunk_by_type(&chunk_type.to_string())
        .ok_or(Error::ChunkTypeNotFound(chunk_type.to_string()))?;

    let data = chunk.data_as_string()?;

    Ok(data)
}

pub fn remove(file_path: PathBuf, chunk_type: ChunkType) -> anyhow::Result<()> {
    let contents = fs::read(&file_path)?;
    let mut png = Png::try_from(contents.as_slice())?;

    let chunk = png
        .chunk_by_type(&chunk_type.to_string())
        .ok_or(Error::ChunkTypeNotFound(chunk_type.to_string()))?;

    png.remove_chunk(&chunk_type.to_string())?;
    let clean_png = png.as_bytes();

    fs::write(file_path, clean_png)?;
    Ok(())
}
