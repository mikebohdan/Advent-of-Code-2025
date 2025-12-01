use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[arg(short, long)]
  pub part: Part,
  #[arg(short, long)]
  pub file: PathBuf,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Part {
  First,
  Second,
}
