pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();

    if n == 1 {
      return 1;
    }

    let mut count = n as i32;

    for i in 0..n {
      if i > 0 {
        let mut l = i - 1;
        let mut r = i + 1;
        while r < n && s[l] == s[r] {
          count += 1;
          r += 1;
          if l > 0 {
            l -= 1;
          } else {
            break;
          }
        }
      }
    }

    for i in 0..n - 1 {
      if s[i] == s[i + 1] {
        count += 1;
        if i > 0 {
          let mut l = i - 1;
          let mut r = i + 2;
          while r < n && s[l] == s[r] {
            count += 1;
            r += 1;
            if l > 0 {
              l -= 1;
            } else {
              break;
            }
          }
        }
      }
    }

    count
  }
}

pub struct SpaceConsumingSolution;

#[allow(dead_code)]
impl SpaceConsumingSolution {
  // Ranging the string S from i to j:
  // S[i][i] is palindromic for every i
  // S[i][i+1] is palindromic if S[i] == S[i+1]
  // S[j][j] is palindromic if S[i] == S[j] and S[i+1][j-1] is palindromic
  pub fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();

    if n == 1 {
      return 1;
    }

    let mut is_palin: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
    let mut count = n as i32;

    for i in 0..n {
      is_palin[i][i] = 1;
    }

    for i in 0..n - 1 {
      if s[i] == s[i + 1] {
        is_palin[i][i + 1] = 1;
        count += 1;
      }
    }

    for l in 3..=n {
      for i in 0..=(n - l) {
        let j = i + l - 1;
        if s[i] == s[j] && is_palin[i + 1][j - 1] > 0 {
          is_palin[i][j] = 1;
          count += 1;
        }
      }
    }

    count
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    let s = "abc".to_string();
    assert_eq!(Solution::count_substrings(s), 3);
    // Explanation: "a", "b", "c"
  }

  #[test]
  fn test_example_2() {
    let s = "aaa".to_string();
    assert_eq!(Solution::count_substrings(s), 6);
    // Explanation: "a", "a", "a", "aa", "aa", "aaa"
  }

  #[test]
  fn test_single_char() {
    let s = "a".to_string();
    assert_eq!(Solution::count_substrings(s), 1);
  }

  #[test]
  fn test_two_same_chars() {
    let s = "aa".to_string();
    assert_eq!(Solution::count_substrings(s), 3);
    // Explanation: "a", "a", "aa"
  }

  #[test]
  fn test_palindrome() {
    let s = "racecar".to_string();
    assert_eq!(Solution::count_substrings(s), 10);
    // "r", "a", "c", "e", "c", "a", "r", "cec", "aceca", "racecar"
  }

  #[test]
  fn test_no_multi_char_palindromes() {
    let s = "abcd".to_string();
    assert_eq!(Solution::count_substrings(s), 4);
    // Only single characters are palindromes
  }

  #[test]
  fn test_alternating() {
    let s = "aba".to_string();
    assert_eq!(Solution::count_substrings(s), 4);
    // "a", "b", "a", "aba"
  }

  #[test]
  fn test_leetcode() {
    let s = "leetcode".to_string();
    assert_eq!(Solution::count_substrings(s), 9);
  }
}
