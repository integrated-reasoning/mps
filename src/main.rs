mod cli;
use clap::Parser;
use cli::Cli;
use color_eyre::{eyre::eyre, Result};
use mps::reader::Reader;
use std::fs;
use std::path::PathBuf;
cfg_if::cfg_if! {
  if #[cfg(feature = "located")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;
  }
}

// TODO: Support comments, see docs

fn main() -> Result<()> {
  let args = Cli::parse();
  cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
        let info = TracableInfo::new().forward(true).backward(true);
        let contents = fs::read_to_string(args.input_path.clone())?;
        match mps::Parser::<f32>::parse(LocatedSpan::new_extra(&contents, info)) {
          Ok((_, parsed)) => Ok(println!("{:#?}", parsed)),
          Err(e) => Err(eyre!(e.to_string())),
        }?;
        nom_tracable::cumulative_histogram();
      } else {
        let path = PathBuf::from(args.input_path.clone());
        let mut r = Reader::try_from(path)?;
        let parsed = r.read()?;
        println!("{:#?}", parsed);
      }
  }
  Ok(())
}
