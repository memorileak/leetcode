use std::collections::HashMap;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    for n in nums.into_iter() {
      let c = count_map.entry(n).or_insert(0);
      *c += 1;
    }

    let mut lhs: i32 = 0;

    for (&n, &c) in count_map.iter() {
      let mut lhs_candidate = 0;
      if let Some(&count_n_plus_1) = count_map.get(&(n+1)) {
        lhs_candidate = c + count_n_plus_1;
      }
      lhs = lhs.max(lhs_candidate);
    }

    lhs
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_example_1() {
    // [1,3,2,2,5,2,3,7] should return 5
    // The longest harmonious subsequence is [3,2,2,2,3]
    let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
    assert_eq!(Solution::find_lhs(nums), 5);
  }

  #[test]
  fn test_example_2() {
    // [1,2,3,4] should return 2
    // The longest harmonious subsequences are [1,2], [2,3], and [3,4]
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::find_lhs(nums), 2);
  }

  #[test]
  fn test_example_3() {
    // [1,1,1,1] should return 0
    // No harmonious subsequence exists
    let nums = vec![1, 1, 1, 1];
    assert_eq!(Solution::find_lhs(nums), 0);
  }
}
