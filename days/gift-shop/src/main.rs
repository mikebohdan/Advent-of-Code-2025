mod range;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};

use crate::range::is_n_silly;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[arg(short, long)]
  pub part:      Part,
  #[arg(short, long)]
  pub file_path: PathBuf,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Part {
  First,
  Second,
}

struct Range {
  min_bound: u64,
  max_bound: u64,
}

fn main() -> Result<()> {
  let args = Args::parse();

  match args.part {
    Part::First => {
      let result: u64 = read_file(args.file_path.as_path())?
        .flat_map(|r| range::silly_patterns(r.min_bound, r.max_bound))
        .sum();

      println!("Sum of all the invalid IDs: {result}");
    }
    Part::Second => {
      let result: u64 = read_file(args.file_path.as_path())?
        .map(|r| {
          let part_sizes = range::possible_parts(r.min_bound, r.max_bound);

          part_sizes
            .iter()
            .enumerate()
            .map(|(i, &part_size)| {
              range::silly_n_pattern(r.min_bound, r.max_bound, part_size)
                .filter(|&x| !part_sizes.iter().take(i).any(|&ps| is_n_silly(x, ps)))
                .sum::<u64>()
            })
            .sum::<u64>()
        })
        .sum();

      println!("Sum of all the invalid IDs: {result}");
    }
  }

  Ok(())
}

fn read_file(file_path: &Path) -> Result<impl Iterator<Item = Range>> {
  let file = File::open(file_path).with_context(|| "cannot open file.")?;
  Ok(
    BufReader::new(file)
      .split(b',')
      .map(|s| {
        let range_str = s.unwrap();
        let mut split = range_str.split(|&x| b'-' == x);
        let lower = unsafe {
          str::from_utf8_unchecked(split.next().unwrap())
            .trim()
            .parse::<u64>()
            .unwrap()
        };
        let upper = unsafe {
          str::from_utf8_unchecked(split.next().unwrap())
            .trim()
            .parse::<u64>()
            .unwrap()
        };
        Range {
          min_bound: lower,
          max_bound: upper,
        }
      })
      .into_iter(),
  )
}
