pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // To solve this problem, imagine we need to compute the minimum number of operations
  // to turn a source string source[0..=i] into a target string target[0..=j].
  //
  // If source[i] == target[j], number of operations is the same as that needed to turn
  // source[0..=i-1] into target[0..=j-1].
  //   min_distance(i, j) = min_distance(i-1, j-1)
  //
  // If source[i] != target[j], we have three options:
  //   1. We turn source[0..=i-1] into target[0..=j] and then delete the character source[i].
  //      This takes min_distance(i-1, j) + 1 operations:
  //   2. We turn source[0..=i] into target[0..=j-1] and then insert the character target[j].
  //      This takes min_distance(i, j-1) + 1 operations.
  //   3. We turn source[0..=i-1] into target[0..=j-1] and then replace the character source[i] with target[j].
  //      This takes min_distance(i-1, j-1) + 1 operations.
  // The minimum number of operations is the minimum of these three options:
  //   min_distance(i, j) = 1 + min(min_distance(i-1, j), min_distance(i, j-1), min_distance(i-1, j-1))
  //
  // Base cases:
  //   min_distance(i, 0) = i -> deleting all i characters from source to produce an empty target string
  //   min_distance(0, j) = j -> inserting all j characters to an empty source string to produce target

  pub fn min_distance(word1: String, word2: String) -> i32 {
    let s = word1.as_bytes();
    let t = word2.as_bytes();

    let ls = s.len();
    let lt = t.len();

    let mut row1: Vec<u16> = vec![0; lt + 1];
    let mut row2: Vec<u16> = vec![0; lt + 1];
    let mut cr = &mut row1;
    let mut pr = &mut row2;

    for j in 1..=lt {
      pr[j] = j as u16;
    }

    for i in 1..=ls {
      cr[0] = i as u16;
      let s_char = s[i - 1];
      for j in 1..=lt {
        if s_char == t[j - 1] {
          cr[j] = pr[j - 1];
        } else {
          cr[j] = 1 + pr[j].min(cr[j - 1]).min(pr[j - 1]);
        }
      }

      std::mem::swap(&mut cr, &mut pr);
    }

    pr[lt] as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution() {
    // Example 1
    assert_eq!(
      Solution::min_distance("horse".to_string(), "ros".to_string()),
      3
    );

    // Example 2
    assert_eq!(
      Solution::min_distance("intention".to_string(), "execution".to_string()),
      5
    );
  }
}
