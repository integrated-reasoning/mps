use crate::types::*;
use color_eyre::{eyre::eyre, Result};
use std::fs::read_to_string;
use std::marker::PhantomData;
use std::path::PathBuf;
cfg_if::cfg_if! {
  if #[cfg(feature = "located")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;
  }
}

#[derive(Debug, Default, Clone)]
pub struct Reader<'a, 'b: 'a> {
  parser: Option<Parser<'a, f32>>,
  input: String,
  _phantom: PhantomData<&'b ()>,
}

impl<'a, 'b: 'a> TryFrom<PathBuf> for Reader<'a, 'b> {
  type Error = color_eyre::Report;

  fn try_from(path: PathBuf) -> Result<Reader<'a, 'b>> {
    Ok(Reader {
      input: read_to_string(path)?,
      ..Default::default()
    })
  }
}

impl<'a, 'b: 'a> Reader<'a, 'b> {
  pub fn read(&'b mut self) -> Result<Reader<'a, 'b>> {
    let parsed = match Parser::<f32>::parse(&self.input) {
      Ok((_, parsed)) => Ok(parsed),
      Err(e) => Err(eyre!(e.to_string())),
    }?;
    self.parser = Some(parsed);
    Ok(self.to_owned())
  }
}
