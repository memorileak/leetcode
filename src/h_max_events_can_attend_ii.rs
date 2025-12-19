pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Assume i is 1-indexed.
  // At event i, [startDayi, endDayi, valuei], you could attend from 1 up to Min(i, k) events.
  // Call V[i][a] the maximum value you can receive by attending a events among the first i events.
  // Then, for each a from 1 to Min(i, k):
  //   V[i][a] = Max(V[i-1][a], V[j][a-1] + valuei) (at some j < i where event j ends before event i starts)
  //   Note: V[j][a-1] is the maximum value you can receive by attending a-1 events among the first j events.
  //   We also don't need to loop `a` strictly to Min(i, k), we can stop at a = Min(Min(i, k), 1 + maximum a at j point).
  // We need to find the j point efficiently.
  // We can sort events by end day, and use binary search to find the largest j where endDayj < startDayi.

  pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut sorted_events = events;
    sorted_events.sort_by(|a, b| a[1].cmp(&b[1])); // Sort by end day

    let mut v: Vec<Vec<i32>> = vec![vec![0; (k as usize) + 1]; sorted_events.len()];
    let mut max_attendable: Vec<usize> = vec![1; sorted_events.len()];

    v[0][1] = sorted_events[0][2];
    let mut max_value: i32 = v[0][1];

    for i in 1..sorted_events.len() {
      let value_i = sorted_events[i][2];

      v[i][1] = v[i - 1][1].max(value_i);
      max_value = max_value.max(v[i][1]);

      // Assume that we don't attend event i
      max_attendable[i] = max_attendable[i - 1];
      for a in 2..=max_attendable[i] {
        v[i][a] = v[i - 1][a];
      }

      // Now consider attending event i
      if let Some(j) = Solution::search_latest_non_overlap(&sorted_events, i) {
        let min_i_k = (i + 1).min(k as usize);
        let clamped_new_max_attendable = min_i_k.min(max_attendable[j] + 1);
        max_attendable[i] = max_attendable[i].max(clamped_new_max_attendable);
        for a in 2..=max_attendable[i] {
          let value_if_attend = v[j][a - 1] + value_i;
          v[i][a] = v[i][a].max(value_if_attend);
          max_value = max_value.max(v[i][a]);
        }
      }
    }

    max_value
  }

  pub fn search_latest_non_overlap(sorted_events: &Vec<Vec<i32>>, i: usize) -> Option<usize> {
    let start_i = sorted_events[i][0];
    return Solution::binary_search_events_end(sorted_events, 0, (i as isize) - 1, start_i);
  }

  pub fn binary_search_events_end(
    sorted_events: &Vec<Vec<i32>>,
    left: isize,
    right: isize,
    target: i32,
  ) -> Option<usize> {
    // Compare the end day of mid event with target
    // If end day of mid event >= target, go left, exclude mid
    // If end day of mid event < target, go right, include mid
    // Until left == right (could be found) or left > right (not found)

    let mut l = left;
    let mut r = right;

    while l <= r {
      if l == r {
        if sorted_events[l as usize][1] < target {
          return Some(l as usize);
        } else {
          return None;
        }
      }
      let m: isize = (1 + l + r) / 2; // Bias to the right to prevent infinite loop
      if sorted_events[m as usize][1] < target {
        l = m;
      } else {
        r = m - 1;
      }
    }

    return None;
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_binary_search_events_end() {
    let sorted_events = vec![
      vec![1, 2, 4],
      vec![2, 3, 1],
      vec![3, 4, 3],
      vec![4, 5, 5],
      vec![5, 6, 8],
    ];
    assert_eq!(
      Solution::binary_search_events_end(&sorted_events, 0, 4, 1),
      None
    );
    assert_eq!(
      Solution::binary_search_events_end(&sorted_events, 0, 4, 3),
      Some(0)
    );
    assert_eq!(
      Solution::binary_search_events_end(&sorted_events, 0, 4, 5),
      Some(2)
    );
    assert_eq!(
      Solution::binary_search_events_end(&sorted_events, 0, 4, 6),
      Some(3)
    );
    assert_eq!(
      Solution::binary_search_events_end(&sorted_events, 0, 4, 10),
      Some(4)
    );
  }

  #[test]
  fn test_max_value() {
    let events1 = vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]];
    let k1 = 2;
    assert_eq!(Solution::max_value(events1, k1), 7);

    let events2 = vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]];
    let k2 = 2;
    assert_eq!(Solution::max_value(events2, k2), 10);

    let events3 = vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]];
    let k3 = 3;
    assert_eq!(Solution::max_value(events3, k3), 9);

    let events4 = vec![
      vec![19, 42, 7],
      vec![41, 73, 15],
      vec![52, 73, 84],
      vec![84, 92, 96],
      vec![6, 64, 50],
      vec![12, 56, 27],
      vec![22, 74, 44],
      vec![38, 85, 61],
    ];
    let k4 = 5;
    assert_eq!(Solution::max_value(events4, k4), 187);
  }
}
