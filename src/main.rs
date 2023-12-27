mod cli;
use color_eyre::{eyre::eyre, Result};
mod file;
use clap::Parser;
use cli::Cli;
use file::MPSFile;
use std::fs;
cfg_if::cfg_if! {
  if #[cfg(feature = "located")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;
  }
}

fn main() -> Result<()> {
  let args = Cli::parse();
  let contents = fs::read_to_string(args.input_path)?;
  cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
        let info = TracableInfo::new().forward(true).backward(true);
        match MPSFile::<f32>::parse(LocatedSpan::new_extra(&contents, info)) {
          Ok((_, parsed)) => Ok(println!("{:#?}", parsed)),
          Err(e) => Err(eyre!(e.to_string())),
        }?;
        nom_tracable::cumulative_histogram();
      } else {
        match MPSFile::<f32>::parse(&contents) {
          Ok((_, parsed)) => Ok(println!("{:#?}", parsed)),
          Err(e) => Err(eyre!(e.to_string())),
        }?;
      }
  }
  Ok(())
}
