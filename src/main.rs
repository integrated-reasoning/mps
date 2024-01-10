mod cli;
use clap::Parser;
use cli::Cli;
use color_eyre::{eyre::eyre, Result};
use std::fs;
cfg_if::cfg_if! {
  if #[cfg(feature = "trace")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;
  }
}

// TODO: Support comments, see docs

fn main() -> Result<()> {
  let args = Cli::parse();
  let contents = fs::read_to_string(args.input_path)?;
  cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let info = TracableInfo::new().forward(true).backward(true);
        match mps::Parser::<f32>::parse(LocatedSpan::new_extra(&contents, info)) {
          Ok((_, parsed)) => Ok(println!("{:#?}", parsed)),
          Err(e) => Err(eyre!(e.to_string())),
        }?;
        nom_tracable::cumulative_histogram();
      } else {
        match mps::Parser::<f32>::parse(&contents) {
          Ok((_, parsed)) => Ok(println!("{:#?}", parsed)),
          Err(e) => Err(eyre!(e.to_string())),
        }?;
      }
  }
  Ok(())
}
