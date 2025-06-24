pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let mut indices: Vec<i32> = vec![];
    let mut accept_from: i32 = 0;

    for i in 0..nums.len() {
      if nums[i] == key {
        let s = accept_from.max(i as i32 - k);
        let e = (nums.len() as i32).min(i as i32 + k + 1);
        indices.append(&mut (s..e).collect::<Vec<i32>>());
        accept_from = e; // Update the next starting point
      }
    }

    indices
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_solution() {
    // Example 1
    assert_eq!(
      Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
      vec![1, 2, 3, 4, 5, 6]
    );
    // Example 2
    assert_eq!(
      Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
      vec![0, 1, 2, 3, 4]
    );
    // Example 3
    assert_eq!(
      Solution::find_k_distant_indices(vec![1, 2, 3, 4, 5], 1, 0),
      vec![0]
    );
    // Example 4
    assert_eq!(
      Solution::find_k_distant_indices(vec![1, 2, 3, 4, 5], 6, 1),
      vec![]
    );
    // Additional test: Empty array
    assert_eq!(Solution::find_k_distant_indices(vec![], 1, 1), vec![]);
    // Additional test: Single element array
    assert_eq!(Solution::find_k_distant_indices(vec![1], 1, 1), vec![0]);
    // Additional test: Key not present in the array
    assert_eq!(
      Solution::find_k_distant_indices(vec![1, 2, 3, 4, 5], 6, 2),
      vec![]
    );
  }
}
