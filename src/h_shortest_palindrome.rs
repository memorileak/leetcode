pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn shortest_palindrome(s: String) -> String {
    if s.is_empty() {
      return s;
    }

    let chars = s.chars().collect::<Vec<char>>();
    let rbound = Solution::rbound_longest_beginning_palindrome(s.as_str());
    let head = chars[rbound as usize + 1..]
      .iter()
      .rev()
      .collect::<String>();
    let mid = chars[0..=rbound as usize].iter().collect::<String>();
    let tail = chars[rbound as usize + 1..].iter().collect::<String>();
    head + &mid + &tail
  }

  // Find the right boundary of the longest possible palindrome which starts from the beginning of the string
  fn rbound_longest_beginning_palindrome(s: &str) -> i32 {
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

      // Update the longest beginning palindrome found
      if p[_i] == i && max_palindr_radius < p[_i] {
        max_palindr_radius = p[_i];
        max_palindr_center = i;
      }
    }

    // Extract right-boundary of the longest palindrome from the alt_s vector
    let max_palindr_end: i32 = max_palindr_center + max_palindr_radius;
    let rbound: i32 = if max_palindr_end % 2 == 0 {
      max_palindr_end / 2 - 1
    } else {
      max_palindr_end / 2
    };
    rbound
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
  use super::*;

  #[test]
  fn test_shortest_palindrome() {
    let s = "aacecaaa".to_string();
    let result = Solution::shortest_palindrome(s);
    assert_eq!(result, "aaacecaaa".to_string());

    let s = "abcd".to_string();
    let result = Solution::shortest_palindrome(s);
    assert_eq!(result, "dcbabcd".to_string());
  }
}
