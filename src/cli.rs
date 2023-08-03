use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{chunk_type::ChunkType, commands};

use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encode {
        #[arg(short, long, help = "Input file path.")]
        file_path: PathBuf,
        #[arg(short, long, default_value_t = ChunkType::from_str("STXT").unwrap(),  help = "Key to store message as.")]
        chunk_type: ChunkType,
        #[arg(short, long, help = "Message to be stored.")]
        msg: String,
        #[arg(short, long, help = "Output file.")]
        output_file: Option<PathBuf>,
    },

    Decode {
        #[arg(short, long, help = "Input file path.")]
        file_path: PathBuf,
        #[arg(short, long, default_value_t = ChunkType::from_str("STXT").unwrap(), help = "Key to store message as.")]
        chunk_type: ChunkType,
    },

    Remove {
        #[arg(short, long, help = "Input file path.")]
        file_path: PathBuf,
        #[arg(short, long, default_value_t = ChunkType::from_str("STXT").unwrap(), help = "Key to store message as.")]
        chunk_type: ChunkType,
    },
}

pub fn parse() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            msg,
            output_file,
        } => commands::encode(file_path, chunk_type, msg, output_file)?,
        Commands::Decode {
            file_path,
            chunk_type,
        } => {
            let data = commands::decode(file_path, chunk_type)?;
            for msg in data {
                println!("{}", msg);
            }
        }
        Commands::Remove {
            file_path,
            chunk_type,
        } => commands::remove(file_path, chunk_type)?,
    };

    Ok(())
}
