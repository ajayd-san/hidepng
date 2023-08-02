mod cli;
mod chunk_type;
mod chunk;
mod errors;
mod png;
mod commands;

fn main() -> anyhow::Result<()>{
    cli::parse()?;
    Ok(())
}
