use std::io::BufRead;

use anyhow::Result;

use super::cell::Cell;

enum Arith {
  Add(usize),
  Sub(usize),
}

impl Arith {
  fn checked_ap(&self, other: usize, max_size: usize) -> Option<usize> {
    match self {
      Arith::Add(n) => other.checked_add(*n),
      Arith::Sub(n) => {
        let val = other.checked_sub(*n)?;
        if val >= max_size { None } else { Some(val) }
      }
    }
  }
}

static NEIGHBORS: [(Arith, Arith); 8] = [
  // top row
  (Arith::Sub(1), Arith::Sub(1)),
  (Arith::Sub(1), Arith::Add(0)),
  (Arith::Sub(1), Arith::Add(1)),
  // same row
  (Arith::Add(0), Arith::Sub(1)),
  (Arith::Add(0), Arith::Add(1)),
  // next row
  (Arith::Add(1), Arith::Sub(1)),
  (Arith::Add(1), Arith::Add(0)),
  (Arith::Add(1), Arith::Add(1)),
];

#[derive(PartialEq, Eq, Debug)]
pub struct Warehouse {
  plan: Vec<Vec<Cell>>,
}

impl Warehouse {
  pub fn from_read_buf<B>(buf: B) -> Result<Self>
  where
    B: BufRead,
  {
    let mut plan = Vec::new();
    for line in buf.lines() {
      plan.push(Warehouse::parse_row(&line?)?);
    }

    Ok(Warehouse { plan })
  }

  pub fn count_removable_rolls(&mut self) -> u32 {
    let mut removable_rolls = 0;
    let mut q = self.find_empty();

    while let Some((i, j)) = q.pop() {
      removable_rolls += self.update_neighbors(i, j, &mut q);
    }

    removable_rolls
  }

  fn update_cell(&mut self, i: usize, j: usize, q: &mut Vec<(usize, usize)>) -> u32 {
    if self.plan[i][j].add_empty_neighbor() {
      q.push((i, j));
      1
    } else {
      0
    }
  }

  fn update_neighbors(&mut self, i: usize, j: usize, q: &mut Vec<(usize, usize)>) -> u32 {
    let max_rows = self.plan.len();
    let max_colls = self.plan[0].len();
    NEIGHBORS
      .iter()
      .map(|(di, dj)| {
        let ni = di.checked_ap(i, max_rows)?;
        let nj = dj.checked_ap(j, max_colls)?;

        Some((ni, nj))
      })
      .filter(|n| n.is_some())
      .map(|n| n.unwrap())
      .map(|(i, j)| self.update_cell(i, j, q))
      .sum()
  }

  fn parse_row(s: &str) -> Result<Vec<Cell>> { s.chars().map(Cell::try_from).collect() }

  fn find_empty(&self) -> Vec<(usize, usize)> {
    self
      .plan
      .iter()
      .enumerate()
      .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, cell)| (i, j, cell)))
      .filter(|(_, _, cell)| cell.is_empty())
      .map(|(i, j, _)| (i, j))
      .collect()
  }
}

#[cfg(test)]
mod test {
  use std::io::Cursor;

  use super::*;

  #[test]
  fn parse_row() {
    let example_wh = Warehouse {
      plan: vec![
        vec![Cell::Empty, Cell::Empty, Cell::Roll(0)],
        vec![Cell::Roll(0), Cell::Roll(0), Cell::Roll(0)],
        vec![Cell::Empty, Cell::Empty, Cell::Empty],
      ],
    };

    let buf = Cursor::new("..@\n@@@\n...");
    let parsed = Warehouse::from_read_buf(buf);
    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(), example_wh);
  }
}
