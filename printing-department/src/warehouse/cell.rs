use anyhow::{Result, anyhow};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Cell {
  Empty,
  Roll(usize),
}

impl Cell {
  pub fn is_empty(&self) -> bool { Cell::Empty == *self }

  pub fn add_empty_neighbor(&mut self) -> bool {
    let Cell::Roll(empty_neighbor_count) = self else {
      // An empty cell cannot became accessible
      return false
    };
    *empty_neighbor_count += 1;

    // Does cell became accessible
    5 == *empty_neighbor_count
  }

  pub fn adjust_neighbor(&mut self, neighbor: &mut Cell) -> u32 {
    if self.is_empty() && neighbor.add_empty_neighbor() {
      return 1;
    }
    if neighbor.is_empty() && self.add_empty_neighbor() {
      return 1;
    }

    0
  }
}

impl TryFrom<char> for Cell {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self> {
    match value {
      '.' => Ok(Cell::Empty),
      '@' => Ok(Cell::Roll(0)),
      c => Err(anyhow!("Unexpected character: `{:?}`", c)),
    }
  }
}
