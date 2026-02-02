use std::{
  collections::HashMap,
  hash::{BuildHasherDefault, Hasher},
};

#[derive(Default)]
pub struct U64Hasher(u64);

impl U64Hasher {
  // Constant from FxHash
  const K: u64 = 0x517cc1b727220a95;
}

impl Hasher for U64Hasher {
  fn write(&mut self, _: &[u8]) {
    unreachable!("Only for u64");
  }

  fn write_u64(&mut self, i: u64) {
    // Cheaply mix the input bits
    // To avoid trivial collisions for sequential keys
    self.0 = i.wrapping_mul(Self::K);
  }

  fn finish(&self) -> u64 {
    self.0
  }
}

pub type U64Map<V> = HashMap<u64, V, BuildHasherDefault<U64Hasher>>;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // W[i][t]: Number of ways to produce target t from numbers in range [0..=i]
  // W[i][t] = W[i-1][t-nums[i]] + W[i-1][t+nums[i]]
  // W[0][t] = 2 if t == nums[0] == 0
  //         = 1 if |t| > 0 and (t == nums[0] or t == -nums[0])
  //         = 0 otherwise
  pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut w: U64Map<i32> = U64Map::default();
    Self::top_down_w(&nums, &mut w, nums.len() - 1, target)
  }

  fn top_down_w(nums: &Vec<i32>, w: &mut U64Map<i32>, i: usize, t: i32) -> i32 {
    let k: u64 = ((i as u64) << 32) | ((t as u32) as u64);

    if let Some(pre_calculated) = w.get(&k) {
      return *pre_calculated;
    }

    let mut v: i32 = 0;
    if i == 0 {
      if t == nums[0] {
        v += 1;
      }
      if t == -nums[0] {
        v += 1;
      }
    } else {
      v = Self::top_down_w(nums, w, i - 1, t - nums[i])
        + Self::top_down_w(nums, w, i - 1, t + nums[i]);
    }
    w.insert(k, v);
    v
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_target_sum_ways() {
    let mut nums = vec![1, 1, 1, 1, 1];
    let mut target = 3;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 5);

    nums = vec![1, 1, 1, 1, 1];
    target = -1000;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 0);

    nums = vec![9, 7, 0, 3, 9, 8, 6, 5, 7, 6];
    target = 2;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 40);
  }
}
