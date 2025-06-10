use std::collections::HashMap;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut needs: HashMap<i32, i32> = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
      if needs.contains_key(n) {
        result.push(*needs.get(n).unwrap());
        result.push(i as i32);
        break;
      } else {
        let complement: i32 = target - *n;
        needs.insert(complement, i as i32);
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 1]);

    let nums = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![1, 2]);

    let nums = vec![3, 3];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 1]);
  }
}
