pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Finds the longest palindromic substring in a given string.
  // Using Manacher's algorithm, which is the most efficient known algorithm for this problem.
  pub fn longest_palindrome(s: String) -> String {
    let mut alt_s: Vec<char> = Vec::with_capacity(s.len() * 2 + 1);
    for c in s.chars() {
      alt_s.push('#');
      alt_s.push(c);
    }
    alt_s.push('#');

    let mut c: i32 = 0; // Center of the current right-most palindrome
    let mut r: i32 = 0; // Right edge of the current right-most palindrome
    let mut p: Vec<i32> = vec![0; alt_s.len()]; // Array to hold the radius of the palindromes center at i
    let mut max_palindr_center: i32 = 0; // Center of the longest palindrome found
    let mut max_palindr_radius: i32 = 0; // Radius of the longest palindrome found

    // Cast i to i32 to avoid overflow issues with large strings
    for _i in 0..alt_s.len() {
      let i: i32 = _i as i32;

      // Out of right-most palindrome range, just launch trivial detection
      if i > r {
        p[_i] = Solution::trivial_odd_palindr_detect(&alt_s, i);
      }
      // Inside the right-most palindrome range, try re-using the previously computed radius
      // Can only utilize the part of the palindrome that is not beyond the left edge of the right-most palindrome
      else {
        let l: i32 = 2 * c - r;
        let m: i32 = 2 * c - i; // Mirror index of i with respect to center c
        let skip = p[m as usize].min(m - l);
        p[_i] = Solution::trivial_odd_palindr_detect_with_skip(&alt_s, i, skip);
      }

      if r < i + p[_i] {
        // We have a new right-most palindrome
        c = i;
        r = i + p[_i];
      }

      // Update the longest palindrome found
      if max_palindr_radius < p[_i] {
        max_palindr_radius = p[_i];
        max_palindr_center = i;
      }
    }

    // Extract the longest palindrome from the alt_s vector
    let max_palindr_start: usize = (max_palindr_center - max_palindr_radius) as usize;
    let max_palindr_end: usize = (max_palindr_center + max_palindr_radius) as usize;
    alt_s[max_palindr_start..max_palindr_end + 1]
      .into_iter()
      .filter(|&c| *c != '#')
      .collect::<String>()
  }

  fn trivial_odd_palindr_detect(chars: &Vec<char>, center: i32) -> i32 {
    Solution::trivial_odd_palindr_detect_with_skip(chars, center, 0)
  }

  fn trivial_odd_palindr_detect_with_skip(chars: &Vec<char>, center: i32, skip: i32) -> i32 {
    let mut radius: i32 = skip;
    let mut d: i32 = skip + 1;
    let mut l: i32 = center - d;
    let mut r: i32 = center + d;
    while l >= 0 && (r as usize) < chars.len() && chars[l as usize] == chars[r as usize] {
      radius += 1;
      d += 1;
      l = center - d;
      r = center + d;
    }
    radius
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  use std::str::Chars;

  #[test]
  fn test_longest_palindrome() {
    let s = "babad".to_string();
    let result = Solution::longest_palindrome(s);
    assert!(result == "bab" || result == "aba");

    let s = "cbbd".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, "bb");

    let s = "a".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, "a");

    let s = "ac".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, "a");

    let s = "racecar".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, "racecar");

    let s = "tacocat".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, "tacocat");

    let s = "eabaeabad".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, "abaeaba");
  }

  #[test]
  fn test_trivial_odd_palindr_detect() {
    let s = Chars::collect::<Vec<char>>("racecar".chars());
    let center = 3;
    let result = Solution::trivial_odd_palindr_detect(&s, center);
    assert_eq!(result, 3);

    let s = Chars::collect::<Vec<char>>("a".chars());
    let center = 0;
    let result = Solution::trivial_odd_palindr_detect(&s, center);
    assert_eq!(result, 0);

    let s = Chars::collect::<Vec<char>>("abcba".chars());
    let center = 2;
    let result = Solution::trivial_odd_palindr_detect(&s, center);
    assert_eq!(result, 2);
  }

  #[test]
  fn test_trivial_odd_palindr_detect_with_skip() {
    let s = Chars::collect::<Vec<char>>("racecar".chars());
    let center = 3;
    let skip = 1;
    let result = Solution::trivial_odd_palindr_detect_with_skip(&s, center, skip);
    assert_eq!(result, 3);

    let s = Chars::collect::<Vec<char>>("a".chars());
    let center = 0;
    let skip = 0;
    let result = Solution::trivial_odd_palindr_detect_with_skip(&s, center, skip);
    assert_eq!(result, 0);

    let s = Chars::collect::<Vec<char>>("abcba".chars());
    let center = 2;
    let skip = 1;
    let result = Solution::trivial_odd_palindr_detect_with_skip(&s, center, skip);
    assert_eq!(result, 2);
  }
}
