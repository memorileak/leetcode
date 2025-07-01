pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn possible_string_count(word: String) -> i32 {
    let mut prev_c: char = '#';
    let mut total_count: i32 = 1;
    for c in word.chars() {
      if c == prev_c {
        total_count += 1;
      }
      prev_c = c;
    }
    total_count
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::possible_string_count("abbcccc".to_string()), 5);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::possible_string_count("abcd".to_string()), 1);
  }

  #[test]
  fn test_example_3() {
    assert_eq!(Solution::possible_string_count("aaaa".to_string()), 4);
  }
}
