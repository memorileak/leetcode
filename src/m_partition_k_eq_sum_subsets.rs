use std::collections::VecDeque;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Before doing any DP, we must check if a solution is mathematically possible.
  // Calculate the total_sum of the array.
  // If total_sum % k != 0, we cannot divide it equally. Return False.
  // Calculate the target sum for each subset: target = total_sum / k.

  // We use an integer (the mask) to represent the state of the nums array.
  // The mask is a sequence of bits (0s and 1s).
  // If the i-th bit is 1, it means nums[i] has been used.
  // If the i-th bit is 0, it means nums[i] is available.

  // We create an array v of size 2^N (number of total subsets).
  // v[mask] stores the current state of the subset currently being built using the elements in mask.
  //   - v[mask] = -1 means the state is invalid (cannot form valid subsets with this mask), cannot continue that path.
  //   - v[mask] = 0 we have perfectly filled one or more subsets and are starting a fresh one.
  //   - v[mask] > 0 we are in the middle of filling a subset, and the value is the current sum.

  // Initialize v with all -1 (invalid).
  // Set v[0] = 0 (empty set has state 0).
  // Use BFS to explore all possible masks.
  // For each mask, try to add each unused number nums[i] (where the i-th bit is 0).
  // Form a new mask by setting the i-th bit to 1.
  // For example: 0000 -> 1000, 0100, 0010, 0001
  //              1000 -> 1100, 1010, 1001
  //              0100 -> 1100, 0110, 0101
  //              1100 -> 1110, 1101
  //              ...
  // Calculate the new value: new_value = v[mask] + nums[i].
  //   If new_value <= target, update v[new_mask] = new_value % target.
  //   If new_value > target, skip this addition (invalid state).
  // Continue until all masks are processed.
  // Finally, check v[2^N - 1] (all elements used).
  //   If v[(1 << N) - 1] == 0, return True (valid partitioning).
  //   Otherwise, return False.

  // Notice that a mask can appear multiple times in our vision of the state space.
  // It may first form an invalid state, but later find a valid way to fill subsets leading to a valid state.
  // For example, with nums = [1,2,3,4] and k = 2: target = 5.
  // The mask 1011 can first be formed via 1 -> 3 -> 4 (sum 8, invalid),
  // But later can be formed via 1 -> 4 -> 3 (sum 3, valid).
  // When the value v[mask] is a valid state, we treat that mask as visited.
  // If not, we just leave it invalid, we may revisit that mask later when we find a valid path.

  // Nums:   [1, 2, 3, 4]
  // Mask:   [0, 0, 0, 0]
  // m = 1   [1, 0, 0, 0]    v = 1
  // m = 2   [0, 1, 0, 0]    v = 2
  // m = 4   [0, 0, 1, 0]    v = 3
  // m = 8   [0, 0, 0, 1]    v = 4
  // m = 3   [1, 1, 0, 0]    v = 3
  // m = 5   [1, 0, 1, 0]    v = 4
  // m = 9   [1, 0, 0, 1]    v = 0
  // m = 6   [0, 1, 1, 0]    v = 0
  // m = 10  [0, 1, 0, 1]    invalid
  // m = 12  [0, 0, 1, 1]    invalid
  // m = 10  [0, 1, 0, 1]    invalid
  // m = 12  [0, 0, 1, 1]    invalid
  // m = 7   [1, 1, 1, 0]    invalid
  // m = 11  [1, 1, 0, 1]    invalid
  // m = 7   [1, 1, 1, 0]    invalid
  // m = 13  [1, 0, 1, 1]    invalid
  // m = 11  [1, 1, 0, 1]    v = 2
  // m = 13  [1, 0, 1, 1]    v = 3
  // m = 7   [1, 1, 1, 0]    v = 1
  // m = 14  [0, 1, 1, 1]    v = 4
  // m = 15  [1, 1, 1, 1]    v = 0

  const N: usize = 1 << 16; // 2^16

  pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    if k == 1 {
      return true;
    }

    let sum: i32 = nums.iter().sum();
    if (sum % k) != 0 {
      return false;
    }

    let target: i32 = sum / k;
    let n = nums.len();
    let mut v: [i32; Self::N] = [-1; Self::N];
    let mut queue: VecDeque<usize> = VecDeque::new();

    v[0] = 0;

    for i in 0..n {
      let k: usize = 1 << i;
      let m: usize = 0 | k;
      let val = v[0] + nums[i];
      if val <= target {
        v[m] = val % target;
        queue.push_back(m);
      }
    }

    while !queue.is_empty() {
      let m = queue.pop_front().unwrap();
      for i in 0..n {
        let k: usize = 1 << i;
        let next_m = m | k;
        if next_m != m && v[next_m] < 0 {
          let next_val = v[m] + nums[i];
          if next_val <= target {
            v[next_m] = next_val % target;
            queue.push_back(next_m);
          }
        }
      }
    }

    v[(1 << n) - 1] == 0
  }

  fn bits_le(n: usize, v: usize) -> String {
    let mut bits: Vec<u8> = vec![0; n];
    let mut v = v;
    for i in 0..n {
      bits[i] = (v & 1) as u8;
      v >>= 1;
    }
    format!("{:?}", bits)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    // It is possible to divide into 4 subsets: (5), (1,4), (2,3), (2,3) with equal sums of 5
    assert_eq!(
      Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4),
      true
    );
  }

  #[test]
  fn test_example_2() {
    // Cannot divide [1,2,3,4] into 3 subsets with equal sums
    assert_eq!(
      Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3),
      false
    );
  }

  #[test]
  fn test_example_3() {
    // Can divide [1,2,3,4] into 2 subsets with equal sums
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 2), true);
  }

  #[test]
  fn test_single_subset() {
    // k=1 means all numbers form one subset
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3], 1), true);
  }

  #[test]
  fn test_impossible_sum() {
    // Total sum is 15, cannot divide evenly into 4 subsets
    assert_eq!(
      Solution::can_partition_k_subsets(vec![1, 2, 3, 4, 5], 4),
      false
    );
  }

  #[test]
  fn test_single_elements() {
    // Each element forms its own subset
    assert_eq!(Solution::can_partition_k_subsets(vec![2, 2, 2, 2], 4), true);
  }

  #[test]
  fn test_element_too_large() {
    // One element (10) is larger than target subset sum (6)
    assert_eq!(
      Solution::can_partition_k_subsets(vec![10, 1, 1, 1, 1, 1, 1, 1, 1], 3),
      false
    );
  }
}
