use std::io::{BufRead, Lines};
use std::iter::Peekable;

use anyhow::Result;

use super::cell::Cell;

pub type WarehouseRow = Vec<Cell>;

pub struct Warehouse<B: BufRead> {
  stop:      bool,
  head_row:  Option<WarehouseRow>,
  next_rows: Peekable<Lines<B>>,
}

impl<B: BufRead> Warehouse<B> {
  pub fn new(buf: B) -> Self {
    Self {
      head_row:  None,
      stop:      false,
      next_rows: buf.lines().peekable(),
    }
  }

  fn parse_row(&mut self, s: &str) -> Result<WarehouseRow> {
    let mut row = self.empty_row();

    let mut row_iter = row.iter_mut();
    row_iter.next();

    for (cell, ch) in row_iter.zip(s.chars()) {
      *cell = Cell::try_from(ch)?;
    }

    Ok(row)
  }

  fn empty_row(&mut self) -> WarehouseRow { vec![Cell::Empty; self.line_size()] }

  fn line_size(&mut self) -> usize {
    match (&self.head_row, self.next_rows.peek()) {
      (None, None) => 0,
      (None, Some(row)) => {
        let Ok(s) = row else {
          return 0;
        };
        s.chars().count() + 2
      }
      (Some(row), _) => row.len(),
    }
  }

  pub fn count_accessible_rolls(&mut self) -> u32 {
    let mut accessibl_rolls = 0;
    self.reduce(|mut prev, mut curr| {
      for i in 0..curr.len() {
        accessibl_rolls += adjust_cell(i, &mut prev, &mut curr);
      }
      curr
    });

    accessibl_rolls
  }
}

impl<B: BufRead> Iterator for Warehouse<B> {
  type Item = WarehouseRow;

  fn next(&mut self) -> Option<Self::Item> {
    match (self.head_row.clone(), self.next_rows.next()) {
      (None, None) => None,
      (None, Some(row_line)) => {
        self.head_row = Some(self.parse_row(&row_line.ok()?).ok()?);
        Some(self.empty_row())
      }
      (Some(head_row), None) => {
        if self.stop {
          self.head_row = None;
        } else {
          self.head_row = Some(self.empty_row());
          self.stop = true;
        };
        Some(head_row)
      }
      (Some(head_row), Some(row_line)) => {
        self.head_row = self.parse_row(&row_line.ok()?).ok();
        Some(head_row)
      }
    }
  }
}

fn adjust_cell(pos: usize, prev_row: &mut [Cell], curr_row: &mut [Cell]) -> u32 {
  let mut newly_accessible = 0;

  let (left, current_and_right) = curr_row.split_at_mut(pos);
  let current_cell = &mut current_and_right[0];

  if let Some(prev_cell) = left.last_mut() {
    newly_accessible += current_cell.adjust_neighbor(prev_cell);
  }

  if let Some(prev_pos) = pos.checked_sub(1) {
    newly_accessible += current_cell.adjust_neighbor(&mut prev_row[prev_pos]);
  }
  newly_accessible += current_cell.adjust_neighbor(&mut prev_row[pos]);
  if pos + 1 < prev_row.len() {
    newly_accessible += current_cell.adjust_neighbor(&mut prev_row[pos + 1]);
  }

  newly_accessible
}

#[cfg(test)]
mod test {
  use std::io::Cursor;

  use super::*;

  #[test]
  fn test_add_empty_neighbor() {
    let mut empty_cell = Cell::Empty;

    assert!(!empty_cell.add_empty_neighbor());

    let mut roll_cell = Cell::Roll(0);

    assert!(!roll_cell.add_empty_neighbor());
    assert_eq!(roll_cell, Cell::Roll(1));
    assert!(!roll_cell.add_empty_neighbor());
    assert_eq!(roll_cell, Cell::Roll(2));
    assert!(!roll_cell.add_empty_neighbor());
    assert_eq!(roll_cell, Cell::Roll(3));
    assert!(!roll_cell.add_empty_neighbor());
    assert_eq!(roll_cell, Cell::Roll(4));
    assert!(roll_cell.add_empty_neighbor());
    assert_eq!(roll_cell, Cell::Roll(5));
    assert!(!roll_cell.add_empty_neighbor());
    assert_eq!(roll_cell, Cell::Roll(6));
    assert!(!roll_cell.add_empty_neighbor());
    assert_eq!(roll_cell, Cell::Roll(7));
    assert!(!roll_cell.add_empty_neighbor());
    assert_eq!(roll_cell, Cell::Roll(8));
  }

  #[test]
  fn test_roll_cell_iter_from_str() {
    let s = String::from(".@a");
    let mut iter = s.chars().map(|c| Cell::try_from(c));

    let first = iter.next().unwrap();
    assert!(first.is_ok());
    assert_eq!(first.unwrap(), Cell::Empty);

    let second = iter.next().unwrap();
    assert!(second.is_ok());
    assert_eq!(second.unwrap(), Cell::Roll(0));

    let third = iter.next().unwrap();
    assert!(third.is_err());
  }

  #[test]
  fn test_adjust_neighbor() {
    let mut left = Cell::Empty;
    let mut right = Cell::Empty;
    assert_eq!(left.adjust_neighbor(&mut right), 0);

    let mut left = Cell::Empty;
    let mut right = Cell::Roll(0);
    assert_eq!(left.adjust_neighbor(&mut right), 0);
    assert_eq!(right, Cell::Roll(1));

    let mut left = Cell::Roll(0);
    let mut right = Cell::Empty;
    assert_eq!(left.adjust_neighbor(&mut right), 0);
    assert_eq!(left, Cell::Roll(1));

    let mut left = Cell::Roll(0);
    let mut right = Cell::Roll(0);
    assert_eq!(left.adjust_neighbor(&mut right), 0);
    assert_eq!(left, Cell::Roll(0));
    assert_eq!(right, Cell::Roll(0));
    // --------------
    let mut left = Cell::Empty;
    let mut right = Cell::Roll(4);
    assert_eq!(left.adjust_neighbor(&mut right), 1);
    assert_eq!(right, Cell::Roll(5));

    let mut left = Cell::Roll(4);
    let mut right = Cell::Empty;
    assert_eq!(left.adjust_neighbor(&mut right), 1);
    assert_eq!(left, Cell::Roll(5));

    let mut left = Cell::Roll(4);
    let mut right = Cell::Roll(4);
    assert_eq!(left.adjust_neighbor(&mut right), 0);
    assert_eq!(left, Cell::Roll(4));
    assert_eq!(right, Cell::Roll(4));
  }

  #[test]
  fn test_parse_row() {
    let buf = Cursor::new("...\n.@.");
    let mut wh = Warehouse::new(buf);

    let row = wh.parse_row("@.@");
    assert!(row.is_ok());
    assert_eq!(row.unwrap(), vec![
      Cell::Empty,
      Cell::Roll(0),
      Cell::Empty,
      Cell::Roll(0),
      Cell::Empty,
    ]);

    let row = wh.parse_row("@..");
    assert!(row.is_ok());
    assert_eq!(row.unwrap(), vec![
      Cell::Empty,
      Cell::Roll(0),
      Cell::Empty,
      Cell::Empty,
      Cell::Empty,
    ]);

    let row = wh.parse_row("@.a");
    assert!(row.is_err());

    let row = wh.parse_row("..@");
    assert!(row.is_ok());
    assert_eq!(row.unwrap(), vec![
      Cell::Empty,
      Cell::Empty,
      Cell::Empty,
      Cell::Roll(0),
      Cell::Empty,
    ]);

    let row = wh.parse_row(".@.");
    assert!(row.is_ok());
    assert_eq!(row.unwrap(), vec![
      Cell::Empty,
      Cell::Empty,
      Cell::Roll(0),
      Cell::Empty,
      Cell::Empty,
    ]);

    let row = wh.parse_row("...");
    assert!(row.is_ok());
    assert_eq!(row.unwrap(), vec![
      Cell::Empty,
      Cell::Empty,
      Cell::Empty,
      Cell::Empty,
      Cell::Empty,
    ]);
  }

  #[test]
  fn test_solve_1() {
    let buf = Cursor::new("...\n.@.");
    let mut wh = Warehouse::new(buf);

    assert_eq!(wh.count_accessible_rolls(), 1);
  }

  #[test]
  fn test_solve_example() {
    let buf = Cursor::new(
      "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@\
       .@@@.@.",
    );
    let mut wh = Warehouse::new(buf);

    assert_eq!(wh.count_accessible_rolls(), 13);
  }
}
