use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about = "A utility for parsing MPS files")]
pub struct Cli {
  #[arg(
    short,
    long,
    value_name = "FILE",
    help = "The path to the MPS file to parse"
  )]
  pub input_path: String,
}
