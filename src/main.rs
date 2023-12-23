mod cli;
use color_eyre::{eyre::eyre, Result};
mod file;
use clap::Parser;
use cli::Cli;
use file::MPSFile;
use std::fs;

fn main() -> Result<()> {
  let args = Cli::parse();
  let contents = fs::read_to_string(args.input_path)?;
  match MPSFile::<f32>::parse(&contents) {
    Ok((_, parsed)) => Ok(println!("{:#?}", parsed)),
    Err(e) => Err(eyre!(e.to_string())),
  }?;
  Ok(())
}
