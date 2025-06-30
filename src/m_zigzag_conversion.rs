pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
      return s;
    }

    let mut vec_vec_chars: Vec<Vec<char>> = vec![vec![]; num_rows as usize];
    let mut i: i32 = 0;
    let mut di: i32 = 0;

    for c in s.chars() {
      vec_vec_chars[i as usize].push(c);
      if i == 0 {
        di = 1;
      } else if i == num_rows - 1 {
        di = -1;
      }
      i += di;
    }

    let mut chars: Vec<char> = Vec::with_capacity(s.len() as usize);
    for vec_chars in vec_vec_chars.into_iter() {
      chars.extend(vec_chars.into_iter());
    }

    chars.into_iter().collect::<String>()
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_example_1() {
    assert_eq!(
      Solution::convert("PAYPALISHIRING".to_string(), 3),
      "PAHNAPLSIIGYIR"
    );
  }

  #[test]
  fn test_example_2() {
    assert_eq!(
      Solution::convert("PAYPALISHIRING".to_string(), 4),
      "PINALSIGYAHRPI"
    );
  }

  #[test]
  fn test_example_3() {
    assert_eq!(Solution::convert("A".to_string(), 1), "A");
  }
}
