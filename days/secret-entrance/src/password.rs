use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Result;

static DIAL_SIZE: usize = 100;

#[derive(Debug, PartialEq)]
pub enum Rotation {
  Left(usize),
  Right(usize),
}

impl From<String> for Rotation {
  fn from(value: String) -> Self {
    if value.starts_with('L') {
      return Rotation::Left(value[1..].parse().unwrap())
    }
    if value.starts_with('R') {
      return Rotation::Right(value[1..].parse().unwrap())
    }
    unreachable!()
  }
}

struct Dial {
  pos: usize,
}

impl Dial {
  fn new() -> Self { Dial { pos: 50 } }

  fn is_zero(&self) -> bool { self.pos == 0 }

  fn apply(&mut self, rot: &Rotation) -> usize {
    let curr = self.pos;
    match rot {
      Rotation::Left(dist) => {
        let new_pos = DIAL_SIZE + self.pos - dist % DIAL_SIZE;
        self.pos = new_pos % DIAL_SIZE;
        if curr > 0 && new_pos < DIAL_SIZE {
          dist / DIAL_SIZE + 1
        } else {
          dist / DIAL_SIZE
        }
      }
      Rotation::Right(dist) => {
        let new_pos = self.pos + dist;
        self.pos = new_pos % DIAL_SIZE;
        if self.pos == 0 {
          new_pos / DIAL_SIZE - 1
        } else {
          new_pos / DIAL_SIZE
        }
      }
    }
  }
}

pub fn find_password<F>(count_zeroes: F, path: PathBuf) -> Result<usize>
where
  F: Fn(Box<dyn Iterator<Item = Rotation>>) -> usize,
{
  let rotations = parse_rotations(path)?;
  Ok(count_zeroes(Box::new(rotations)))
}

pub fn count_zeroes(rotations: Box<dyn Iterator<Item = Rotation>>) -> usize {
  let mut dial = Dial::new();
  let mut zeros = 0;

  for rotation in rotations {
    dial.apply(&rotation);
    if dial.is_zero() {
      zeros += 1;
    }
  }
  zeros
}

pub fn count_zeroes_2(rotations: Box<dyn Iterator<Item = Rotation>>) -> usize {
  let mut dial = Dial::new();
  let mut zeros = 0;

  for rotation in rotations {
    zeros += dial.apply(&rotation);
    if dial.is_zero() {
      zeros += 1;
    }
  }
  zeros
}

fn parse_rotations(path: PathBuf) -> Result<impl Iterator<Item = Rotation>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  Ok(
    reader
      .lines()
      .map(|line| line.expect("cannot read line."))
      .map(Rotation::from),
  )
}
