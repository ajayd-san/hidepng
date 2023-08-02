use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::chunk_type::ChunkType;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encode {
        file_path: PathBuf,
        chunk_type: ChunkType,
        msg: String,
        output_file: Option<PathBuf>,
    },

    Decode {
        file_path: PathBuf,
        chunk_type: ChunkType,
    },

    Remove {
        file_path: PathBuf,
        chunk_type: ChunkType,
    },

    Print {
        file_path: PathBuf,
    },
}

pub fn parse() {
    let args = Cli::parse();

    println!("{:?}", args);
}
