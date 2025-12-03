//! ## Part One
//! The solution to the first part is a classic [two-pointers technique](https://www.geeksforgeeks.org/dsa/two-pointers-technique/).
//!
//! ## Part Two
//! According to the task definition, the solution will be a variation of the previous one, with a slight twist: we need 12 pointers.
//! Instead of writing an "exact 12 pointers" solution, we can abstract to an N-pointers solution, where N is a positive number.

static ZERO: u8 = b'0';

pub fn banks_max(bank: &str) -> u64 {
  let bank_bytes = bank.as_bytes();
  let mut i = 0;
  let mut j = 1;
  let mut joltage = 0;

  while j < bank_bytes.len() {
    joltage = joltage.max(calc_joltage_for(bank_bytes, i, j));

    if bank_bytes[i] < bank_bytes[j] {
      i = j;
    }
    j += 1;
  }

  joltage
}

fn calc_joltage_for(bank: &[u8], i: usize, j: usize) -> u64 {
  let tens = (bank[i] - ZERO) as u64;
  let ones = (bank[j] - ZERO) as u64;

  tens * 10 + ones
}

// --- Part Two ---

pub fn banks_n_max(bank: &str, n: usize) -> u64 {
  let bank_bytes = bank.as_bytes();
  let mut batteries_vec: Vec<usize> = (0..n).collect();
  let batteries: &mut [usize] = &mut batteries_vec;
  let mut joltage = 0;

  while batteries[n - 1] < bank_bytes.len() {
    joltage = joltage.max(calc_joltage_n_for(bank_bytes, batteries));

    if let Some(mut i) = (0..n - 1).find(|&i| bank_bytes[batteries[i + 1]] > bank_bytes[batteries[i]]) {
      while i < n - 1 {
        batteries[i] = batteries[i + 1];
        i += 1;
      }
    }
    batteries[n - 1] += 1;
  }

  joltage
}

fn calc_joltage_n_for(bank: &[u8], selected_batteries: &[usize]) -> u64 {
  selected_batteries
    .iter()
    .rev()
    .enumerate()
    .fold(0, |acc, (n, &i)| acc + (bank[i] - ZERO) as u64 * 10u64.pow(n as u32))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_banks_max() {
    assert_eq!(banks_max("987654321111111"), 98);
    assert_eq!(banks_max("811111111111119"), 89);
    assert_eq!(banks_max("234234234234278"), 78);
    assert_eq!(banks_max("818181911112111"), 92);
  }

  #[test]
  fn test_calc_joltage_n_for() {
    assert_eq!(
      calc_joltage_n_for("987654321111111".as_bytes(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
      987654321111
    );
    assert_eq!(
      calc_joltage_n_for("811111111111119".as_bytes(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 14]),
      811111111119
    );
    assert_eq!(
      calc_joltage_n_for("234234234234278".as_bytes(), &[2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]),
      434234234278
    );
  }

  #[test]
  fn test_banks_maxs_equal() {
    assert_eq!(banks_max("987654321111111"), banks_n_max("987654321111111", 2));
    assert_eq!(banks_max("811111111111119"), banks_n_max("811111111111119", 2));
    assert_eq!(banks_max("234234234234278"), banks_n_max("234234234234278", 2));
    assert_eq!(banks_max("818181911112111"), banks_n_max("818181911112111", 2));
  }

  #[test]
  fn test_banks_n_max() {
    assert_eq!(banks_n_max("987654321111111", 12), 987654321111);
    assert_eq!(banks_n_max("811111111111119", 12), 811111111119);
    assert_eq!(banks_n_max("234234234234278", 12), 434234234278);
    assert_eq!(banks_n_max("818181911112111", 12), 888911112111);
  }
}
