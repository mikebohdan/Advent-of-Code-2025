use std::iter;

fn digits(n: u64) -> u32 { if n == 0 { 1 } else { n.ilog10() + 1 } }

fn lower_bound_half(n: u64) -> u64 {
  let d = digits(n);
  let half = d / 2;

  if d % 2 == 1 {
    10_u64.pow(half)
  } else {
    n / 10_u64.pow(half)
  }
}

fn silly_number(half: u64) -> u64 { half * 10_u64.pow(digits(half)) + half }

pub fn silly_patterns(lower_bound: u64, upper_bound: u64) -> impl Iterator<Item = u64> {
  let mut current_half = lower_bound_half(lower_bound);
  iter::from_fn(move || {
    let mut current = silly_number(current_half);
    while current < lower_bound {
      current_half += 1;
      current = silly_number(current_half);
    }

    if current > upper_bound {
      None
    } else {
      current_half += 1;
      Some(current)
    }
  })
}

// --- Part Two ---

fn lower_n_part(num: u64, parts: u32) -> u64 {
  let d = digits(num);
  let part_size = d / parts;

  if d % parts == 0 {
    num / 10_u64.pow(part_size * (parts - 1))
  } else {
    10_u64.pow(part_size)
  }
}

fn silly_n_number(part: u64, n: u32) -> u64 {
  let part_size = digits(part);
  (0..n).fold(0, |acc, part_n| {
    let pow = part_n * part_size;
    acc + part * 10_u64.pow(pow)
  })
}

pub fn silly_n_pattern(lower_bound: u64, upper_bound: u64, n: u32) -> impl Iterator<Item = u64> {
  let mut current_part = lower_n_part(lower_bound, n);
  iter::from_fn(move || {
    let mut current = silly_n_number(current_part, n);
    while current < lower_bound {
      current_part += 1;
      current = silly_n_number(current_part, n);
    }

    if current > upper_bound {
      None
    } else {
      current_part += 1;
      Some(current)
    }
  })
}

pub fn is_n_silly(num: u64, n: u32) -> bool {
  let part = lower_n_part(num, n);
  silly_n_number(part, n) == num
}

pub fn possible_parts(min_boundary: u64, max_boundary: u64) -> Vec<u32> {
  let min_d = digits(min_boundary);
  let max_d = digits(max_boundary);

  (2..=max_d)
    .filter(move |&x| (min_d..=max_d).any(|d| d % x == 0))
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_digits() {
    assert_eq!(digits(0), 1);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(1227775554), 10);
  }

  #[test]
  fn test_lower_bound_half() {
    assert_eq!(lower_bound_half(11), 1);
    assert_eq!(lower_bound_half(95), 9);
    assert_eq!(lower_bound_half(998), 10);
    assert_eq!(lower_bound_half(1188511880), 11885);
    assert_eq!(lower_bound_half(222220), 222);
    assert_eq!(lower_bound_half(1698522), 1000);
    assert_eq!(lower_bound_half(446443), 446);
    assert_eq!(lower_bound_half(38593856), 3859);
  }

  #[test]
  fn test_silly_number() {
    assert_eq!(silly_number(1), 11);
    assert_eq!(silly_number(64), 6464);
    assert_eq!(silly_number(11885), 1188511885)
  }

  #[test]
  fn test_silly_patterns() {
    assert_eq!(silly_patterns(11, 22).collect::<Vec<u64>>(), vec![11, 22]);
    assert_eq!(silly_patterns(95, 115).collect::<Vec<u64>>(), vec![99]);
    assert_eq!(silly_patterns(998, 1012).collect::<Vec<u64>>(), vec![1010]);
    assert_eq!(silly_patterns(1188511880, 1188511890).collect::<Vec<u64>>(), vec![
      1188511885
    ]);
    assert_eq!(silly_patterns(222220, 222224).collect::<Vec<u64>>(), vec![222222]);
    assert_eq!(silly_patterns(1698522, 1698528).collect::<Vec<u64>>(), vec![]);
  }

  // --- Part Two ---

  #[test]
  fn test_lower_n_part() {
    // part_size == 2
    assert_eq!(lower_n_part(11, 2), 1);
    assert_eq!(lower_n_part(95, 2), 9);
    assert_eq!(lower_n_part(998, 2), 10);
    assert_eq!(lower_n_part(1188511880, 2), 11885);
    assert_eq!(lower_n_part(222220, 2), 222);
    assert_eq!(lower_n_part(1698522, 2), 1000);
    assert_eq!(lower_n_part(446443, 2), 446);
    assert_eq!(lower_n_part(38593856, 2), 3859);
    // part_size == 3
    assert_eq!(lower_n_part(11, 3), 1);
    assert_eq!(lower_n_part(95, 3), 1);
    assert_eq!(lower_n_part(998, 3), 9);
    assert_eq!(lower_n_part(118851188, 3), 118);
    assert_eq!(lower_n_part(222220, 3), 22);
    assert_eq!(lower_n_part(1698522, 3), 100);
    assert_eq!(lower_n_part(446443, 3), 44);
    assert_eq!(lower_n_part(38593856, 3), 100);
    // // part_size == 5
    assert_eq!(lower_n_part(11, 5), 1);
    assert_eq!(lower_n_part(95, 5), 1);
    assert_eq!(lower_n_part(998, 5), 1);
    assert_eq!(lower_n_part(1188511880, 5), 11);
    assert_eq!(lower_n_part(222220, 5), 10);
    assert_eq!(lower_n_part(1698522, 5), 10);
    assert_eq!(lower_n_part(446443, 5), 10);
    assert_eq!(lower_n_part(385938563859385, 5), 385);
    // part_size == 7
    assert_eq!(lower_n_part(11, 7), 1);
    assert_eq!(lower_n_part(95, 7), 1);
    assert_eq!(lower_n_part(998, 7), 1);
    assert_eq!(lower_n_part(1188511880, 7), 10);
    assert_eq!(lower_n_part(2222201, 7), 2);
    assert_eq!(lower_n_part(16985221698522, 7), 16);
    assert_eq!(lower_n_part(446443, 7), 1);
    assert_eq!(lower_n_part(38593856, 7), 10);
  }

  #[test]
  fn test_silly_n_number() {
    // two times
    assert_eq!(silly_n_number(1234, 2), 12341234);
    // three times
    assert_eq!(silly_n_number(123, 3), 123123123);
    // five times
    assert_eq!(silly_n_number(12, 5), 1212121212);
    // seven times
    assert_eq!(silly_n_number(1, 7), 1111111);
  }

  #[test]
  fn test_silly_n_pattern() {
    // part_size = 2
    assert_eq!(silly_n_pattern(11, 22, 2).collect::<Vec<u64>>(), vec![11, 22]);
    assert_eq!(silly_n_pattern(95, 115, 2).collect::<Vec<u64>>(), vec![99]);
    assert_eq!(silly_n_pattern(998, 1012, 2).collect::<Vec<u64>>(), vec![1010]);
    assert_eq!(silly_n_pattern(1188511880, 1188511890, 2).collect::<Vec<u64>>(), vec![
      1188511885
    ]);
    assert_eq!(silly_n_pattern(222220, 222224, 2).collect::<Vec<u64>>(), vec![222222]);
    assert_eq!(silly_n_pattern(1698522, 1698528, 2).collect::<Vec<u64>>(), vec![]);
    // part_size = 3
    assert_eq!(silly_n_pattern(11, 22, 3).collect::<Vec<u64>>(), vec![]);
    assert_eq!(silly_n_pattern(95, 115, 3).collect::<Vec<u64>>(), vec![111]);
    assert_eq!(silly_n_pattern(998, 1012, 3).collect::<Vec<u64>>(), vec![999]);
    assert_eq!(silly_n_pattern(1188511880, 1188511890, 3).collect::<Vec<u64>>(), vec![]);
    assert_eq!(silly_n_pattern(222220, 222224, 3).collect::<Vec<u64>>(), vec![222222]);
    assert_eq!(silly_n_pattern(565653, 565659, 3).collect::<Vec<u64>>(), vec![565656]);
    assert_eq!(silly_n_pattern(1698522, 1698528, 3).collect::<Vec<u64>>(), vec![]);
    // part_size = 5
    assert_eq!(silly_n_pattern(95, 115, 5).collect::<Vec<u64>>(), vec![]);
    assert_eq!(silly_n_pattern(998, 12012, 5).collect::<Vec<u64>>(), vec![11111]);
    assert_eq!(silly_n_pattern(1200000000, 1288511890, 5).collect::<Vec<u64>>(), vec![
      1212121212
    ]);
    // part_size = 7
    assert_eq!(silly_n_pattern(95, 115, 7).collect::<Vec<u64>>(), vec![]);
    assert_eq!(silly_n_pattern(998, 2301200, 7).collect::<Vec<u64>>(), vec![
      1111111, 2222222
    ]);
    assert_eq!(
      silly_n_pattern(12000000000000, 12885118900000, 7).collect::<Vec<u64>>(),
      vec![12121212121212]
    );
  }

  #[test]
  fn test_is_n_silly() {
    assert!(is_n_silly(222222, 2));
    assert!(is_n_silly(222222, 3));
    assert!(!is_n_silly(222222, 5));
    assert!(!is_n_silly(222222, 7));
  }

  #[test]
  fn test_possible_parts() {
    assert_eq!(possible_parts(998, 2301200), vec![2, 3, 4, 5, 6, 7]);
    assert_eq!(possible_parts(99812, 2301200), vec![2, 3, 5, 6, 7]);
    assert_eq!(possible_parts(998123, 2301200), vec![2, 3, 6, 7]);
  }
}
