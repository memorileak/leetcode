use std::collections::{HashMap, HashSet};
use std::mem::swap;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
      return vec![];
    }

    let mut found_triplets: Vec<Vec<i32>> = vec![];
    let mut last_seen_target: HashMap<i32, usize> = HashMap::new();
    let mut seen_triplets: HashSet<i64> = HashSet::new();

    for (i, &num) in nums.iter().enumerate().rev() {
      let target = -num;
      if !last_seen_target.contains_key(&target) {
        last_seen_target.insert(target, i);
      }
    }

    for i in 0..nums.len() - 2 {
      for j in i + 1..nums.len() - 1 {
        let target = nums[i] + nums[j];
        if let Some(&k) = last_seen_target.get(&target) {
          if k > j {
            let (key, triplet) = Self::make_triplet(nums[i], nums[j], nums[k]);
            if !seen_triplets.contains(&key) {
              found_triplets.push(triplet);
              seen_triplets.insert(key);
            }
          }
        }
      }
    }

    found_triplets
  }

  // Cause a, b, c are in the range of [-10^5, 10^5],
  // Each number should occupy 20 bits.
  fn make_triplet(a: i32, b: i32, c: i32) -> (i64, Vec<i32>) {
    let mut first = a;
    let mut second = b;
    let mut third = c;
    if first > second {
      swap(&mut first, &mut second);
    }
    if first > third {
      swap(&mut first, &mut third);
    }
    if second > third {
      swap(&mut second, &mut third);
    }
    (
      ((first as i64) << 40) | ((second as i64) << 20) | (third as i64),
      vec![first, second, third],
    )
  }

  // Sorting in linear time using Counting Sort,
  // Knowing the nums contains at most 3000 elements
  // Each element is in the range of [-10^5, 10^5].
  // fn counting_sort(nums: Vec<i32>) -> Vec<i32> {
  //   if nums.is_empty() {
  //     return vec![];
  //   }

  //   let mut min = 100_001;
  //   let mut max = -100_001;

  //   for &num in nums.iter() {
  //     if min > num {
  //       min = num;
  //     }
  //     if max < num {
  //       max = num;
  //     }
  //   }

  //   let length = nums.len();
  //   let range = (max - min + 1) as usize;
  //   let mut count_vec = vec![0; range];

  //   for num in nums.into_iter() {
  //     count_vec[(num - min) as usize] += 1;
  //   }

  //   let mut sorted_nums: Vec<i32> = vec![0; length];
  //   let mut l: usize = 0;
  //   for (i, c) in count_vec.iter().enumerate() {
  //     let mut k: i32 = 0;
  //     while k < *c {
  //       sorted_nums[l] = i as i32 + min;
  //       k += 1;
  //       l += 1;
  //     }
  //   }
  //   sorted_nums
  // }
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn test_counting_sort() {
  //   let nums = vec![3, -3, -1, -2, 1, 0, -1, 2];
  //   let sorted = Solution::counting_sort(nums);
  //   assert_eq!(sorted, vec![-3, -2, -1, -1, 0, 1, 2, 3]);

  //   let nums = vec![1, 2, 3, 4, 5];
  //   let sorted = Solution::counting_sort(nums);
  //   assert_eq!(sorted, vec![1, 2, 3, 4, 5]);

  //   let nums = vec![-5, -4, -3, -2, -1];
  //   let sorted = Solution::counting_sort(nums);
  //   assert_eq!(sorted, vec![-5, -4, -3, -2, -1]);

  //   let nums = vec![];
  //   let sorted = Solution::counting_sort(nums);
  //   assert_eq!(sorted, vec![]);

  //   let nums = vec![100_000, -100_000, 0];
  //   let sorted = Solution::counting_sort(nums);
  //   assert_eq!(sorted, vec![-100_000, 0, 100_000]);

  //   let nums = vec![100_000, 100_000, -100_000, -100_000];
  //   let sorted = Solution::counting_sort(nums);
  //   assert_eq!(sorted, vec![-100_000, -100_000, 100_000, 100_000]);
  // }

  #[test]
  fn test_pairnum_key() {
    assert_eq!(
      Solution::make_triplet(
        0b00000000000000000000000111111111,
        0b00000000000000000000000000011111,
        0b00000000000000000000000000000001
      ),
      (
        0b0000000000000000000000010000000000000001111100000000000111111111,
        vec![
          0b00000000000000000000000000000001,
          0b00000000000000000000000000011111,
          0b00000000000000000000000111111111,
        ]
      )
    );
  }

  #[test]
  fn test_three_sum() {
    // Example 1
    let mut result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
    result.sort(); // Sort for consistent comparison
    let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    expected.sort();
    assert_eq!(result, expected);

    // Example 2
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());

    // Example 3
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);

    // Additional test cases
    let mut result = Solution::three_sum(vec![-2, 0, 0, 2, 2]);
    result.sort();
    let mut expected = vec![vec![-2, 0, 2]];
    expected.sort();
    assert_eq!(result, expected);

    // Empty array
    assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<i32>>::new());

    // Array with less than 3 elements
    assert_eq!(Solution::three_sum(vec![1, 2]), Vec::<Vec<i32>>::new());

    // Array with no solution
    assert_eq!(
      Solution::three_sum(vec![1, 2, 3, 4, 5]),
      Vec::<Vec<i32>>::new()
    );

    // Array with all zeros
    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);

    assert_eq!(
      Solution::three_sum(vec![1, -1, -1, 0]),
      vec![vec![-1, 0, 1]]
    );
  }
}
