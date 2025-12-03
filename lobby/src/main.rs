mod joltage;

use anyhow::Result;
use std::io::BufRead;

use cli_app;

fn main() -> Result<()> {
  cli_app::run(App {})
}

#[derive(Clone, Copy)]
struct App {}

impl cli_app::App for App {
  type Input = Box<dyn Iterator<Item = String>>;

  type Output = u64;

  fn parse_input(self, buf: std::io::BufReader<std::fs::File>) -> anyhow::Result<Self::Input> {
    Ok(Box::new(buf.lines().map(|s| s.unwrap())))
  }

  fn solve_part_one(self, input: Self::Input) -> anyhow::Result<Self::Output> {
    Ok(input.map(|bank| joltage::banks_max(&bank)).sum::<u64>())
  }

  fn solve_part_two(self, input: Self::Input) -> anyhow::Result<Self::Output> {
    Ok(input.map(|bank| joltage::banks_n_max(&bank, 12)).sum::<u64>())
  }
}
