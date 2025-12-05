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
      Arith::Add(n) => {
        let r = other.checked_add(*n)?;
        if r >= max_size { None } else { Some(r) }
      }
      Arith::Sub(n) => other.checked_sub(*n),
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
      let mut row = Warehouse::parse_row(&line?)?;
      row.push(Cell::Empty);
      row.insert(0, Cell::Empty);

      if plan.is_empty() {
        plan.push(vec![Cell::Empty; row.len()]);
      }
      plan.push(row);
    }

    plan.push(vec![Cell::Empty; plan.last().unwrap().len()]);

    Ok(Warehouse { plan })
  }

  pub fn count_removable_rolls(&mut self) -> u32 {
    let mut removable_rolls = 0;
    let mut q = self.find_empty();

    while !q.is_empty() {
      q = self.process_empty(&q);
      removable_rolls += q.len();
    }

    removable_rolls as u32
  }

  fn process_empty(&mut self, q: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let max_rows = self.plan.len();
    let max_colls = self.plan[0].len();

    let mut new_q = Vec::new();

    for (i, j) in q.iter() {
      for (di, dj) in NEIGHBORS.iter() {
        let Some(ni) = di.checked_ap(*i, max_rows) else {
          continue;
        };
        let Some(nj) = dj.checked_ap(*j, max_colls) else {
          continue;
        };
        if self.plan[ni][nj].add_empty_neighbor() {
          new_q.push((ni, nj));
        }
      }
    }

    new_q
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
