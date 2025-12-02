use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Context, Ok, Result};
use clap::{Parser, ValueEnum};

pub trait App: Copy {
  type Input;
  type Output: Display;

  fn parse_input(self, buf: BufReader<File>) -> Result<Self::Input>;
  fn solve_part_one(self, input: Self::Input) -> Result<Self::Output>;
  fn solve_part_two(self, input: Self::Input) -> Result<Self::Output>;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  part:      Part,
  #[arg(short, long)]
  file_path: PathBuf,
}

#[derive(ValueEnum, Clone, Debug)]
enum Part {
  First,
  Second,
}

pub fn run(app: impl App) -> Result<()> {
  let args = Args::parse();
  let file = File::open(args.file_path).with_context(|| "cannot open file.")?;

  let input = app.parse_input(BufReader::new(file))?;

  match args.part {
    Part::First => println!("Result: {}", app.solve_part_one(input)?),
    Part::Second => println!("Result: {}", app.solve_part_two(input)?),
  }

  Ok(())
}
