use std::collections::HashMap;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last_seen: HashMap<char, i32> = HashMap::new();
    let mut best_streak: i32 = 0;
    let mut last_streak: i32 = 0;
    let mut last_streak_start: i32 = 0;

    for (i, c) in s.chars().enumerate() {
      if let Some(seen_idx) = last_seen.get(&c) {
        if *seen_idx >= last_streak_start {
          last_streak_start = *seen_idx + 1;
          last_streak = i as i32 - last_streak_start + 1;
        } else {
          last_streak += 1;
        }
      } else {
        last_streak += 1;
      }
      last_seen.insert(c, i as i32);
      best_streak = best_streak.max(last_streak);
    }

    best_streak
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_length_of_longest_substring() {
    let s = "abcabcbb".to_string();
    let result = Solution::length_of_longest_substring(s);
    assert_eq!(result, 3); // "abc"

    let s = "abba".to_string();
    let result = Solution::length_of_longest_substring(s);
    assert_eq!(result, 2); // "ab" or "ba"

    let s = "bbbbb".to_string();
    let result = Solution::length_of_longest_substring(s);
    assert_eq!(result, 1); // "b"

    let s = "pwwkew".to_string();
    let result = Solution::length_of_longest_substring(s);
    assert_eq!(result, 3); // "wke"

    let s = "tmmzuxt".to_string();
    let result = Solution::length_of_longest_substring(s);
    assert_eq!(result, 5); // "mzuxt"
  }
}
