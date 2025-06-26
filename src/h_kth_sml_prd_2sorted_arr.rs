pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    for n in [
      nums1[0] as i64 * nums2[0] as i64,
      nums1[0] as i64 * nums2[nums2.len() - 1] as i64,
      nums1[nums1.len() - 1] as i64 * nums2[0] as i64,
      nums1[nums1.len() - 1] as i64 * nums2[nums2.len() - 1] as i64,
    ]
    .into_iter()
    {
      if n < min {
        min = n;
      }
      if n > max {
        max = n;
      }
    }
    let mut left: i64 = min as i64;
    let mut right: i64 = max as i64;
    while left <= right {
      let mid = left + (right - left) / 2; // Left-biased mid, very important for correctness
      let count = Self::count_products_less_or_equal(&nums1, &nums2, mid);
      if count < k {
        left = mid + 1;
      } else {
        if right == mid {
          break;
        }
        right = mid;
      }
    }
    left
  }

  fn count_products_less_or_equal(nums1: &Vec<i32>, nums2: &Vec<i32>, target: i64) -> i64 {
    let mut count: i64 = 0;
    for &num1 in nums1.iter() {
      if num1 == 0 {
        if target >= 0 {
          count += nums2.len() as i64;
        }
        continue;
      } else if num1 > 0 {
        let mut found: bool = false;
        let mut left: i32 = 0;
        let mut right: i32 = nums2.len() as i32 - 1;
        while left <= right {
          let mid = (right + left) / 2; // Left-biased mid
          let p = num1 as i64 * nums2[mid as usize] as i64;
          if p <= target {
            left = mid as i32 + 1;
          } else {
            found = true;
            if right == mid {
              break;
            }
            right = mid;
          }
        }
        count += if found {
          left as i64
        } else {
          nums2.len() as i64
        };
      } else {
        let mut found: bool = false;
        let mut left: i32 = 0;
        let mut right: i32 = nums2.len() as i32 - 1;
        while left <= right {
          let mid = (right + left + 1) / 2; // Right-biased mid
          let p = num1 as i64 * nums2[mid as usize] as i64;
          if p <= target {
            right = mid as i32 - 1;
          } else {
            found = true;
            if left == mid {
              break;
            }
            left = mid;
          }
        }
        count += if found {
          nums2.len() as i64 - left as i64 - 1
        } else {
          nums2.len() as i64
        };
      }
    }
    count
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_count_products_less_or_equal() {
    // Example 1
    assert_eq!(
      Solution::count_products_less_or_equal(&vec![2, 5, 7], &vec![3, 4, 5], 22),
      6
    );

    // Example 1
    assert_eq!(
      Solution::count_products_less_or_equal(&vec![2, 5], &vec![3, 4], 8),
      2
    );
  }

  #[test]
  fn test_kth_smallest_product() {
    // Example 1
    assert_eq!(Solution::kth_smallest_product(vec![2, 5], vec![3, 4], 2), 8);

    // Example 2
    assert_eq!(
      Solution::kth_smallest_product(vec![-4, -2, 0, 3], vec![2, 4], 6),
      0
    );

    // Example 3
    assert_eq!(
      Solution::kth_smallest_product(vec![-2, -1, 0, 1, 2], vec![-3, -1, 2, 4, 5], 3),
      -6
    );

    // Example 4
    assert_eq!(
      Solution::kth_smallest_product(vec![-100000, 100000], vec![-100000, 100000], 1),
      -10000000000
    );
  }
}
