mod range;

use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Ok, Result};
use cli_app;

fn main() -> Result<()> { cli_app::run(App {}) }

struct Range {
  min_bound: u64,
  max_bound: u64,
}

#[derive(Clone, Copy)]
struct App {}

impl cli_app::App for App {
  type Input = Box<dyn Iterator<Item = Range>>;
  type Output = u64;

  fn parse_input(self, buf: BufReader<File>) -> Result<Self::Input> {
    Ok(Box::new(buf.split(b',').map(|s| {
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
    })))
  }

  fn solve_part_one(self, input: Self::Input) -> Result<Self::Output> {
    Ok(
      input
        .flat_map(|r| range::silly_patterns(r.min_bound, r.max_bound))
        .sum(),
    )
  }

  fn solve_part_two(self, input: Self::Input) -> Result<Self::Output> {
    Ok(
      input
        .map(|r| {
          let part_sizes = range::possible_parts(r.min_bound, r.max_bound);

          part_sizes
            .iter()
            .enumerate()
            .map(|(i, &part_size)| {
              range::silly_n_pattern(r.min_bound, r.max_bound, part_size)
                .filter(|&x| !part_sizes.iter().take(i).any(|&ps| range::is_n_silly(x, ps)))
                .sum::<u64>()
            })
            .sum::<u64>()
        })
        .sum(),
    )
  }
}
