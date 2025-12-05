mod warehouse;

use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use cli_app;

use crate::warehouse::{accessible_rolls, removable_rolls};

fn main() -> Result<()> {
  cli_app::run(App {})?;
  Ok(())
}

#[derive(Clone, Copy)]
struct App;

impl cli_app::App for App {
  type Input = BufReader<File>;
  type Output = u32;

  fn parse_input(self, buf: BufReader<File>) -> Result<Self::Input> { Ok(buf) }

  fn solve_part_one(self, input: Self::Input) -> Result<Self::Output> {
    Ok(accessible_rolls::Warehouse::new(input).count_accessible_rolls())
  }

  fn solve_part_two(self, input: Self::Input) -> Result<Self::Output> {
    Ok(removable_rolls::Warehouse::from_read_buf(input)?.count_removable_rolls())
  }
}
