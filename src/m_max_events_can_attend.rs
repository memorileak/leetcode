pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub const MAX_EVENTS: i32 = 100_000;

  pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
    let sorted_by_start = Solution::counting_sort_events(events, 0);
    let mut end_day_heap: Vec<i32> = vec![0]; // Dummy 0 to make heap 1-indexed
    let mut attended: i32 = 0;
    let mut today: i32 = sorted_by_start[0][0];
    let mut i: usize = 0;

    while i < sorted_by_start.len() || !Solution::is_heap_empty(&end_day_heap) {
      // Indexing end_day of events that start today
      while i < sorted_by_start.len() && sorted_by_start[i][0] == today {
        Solution::insert_into_heap(&mut end_day_heap, sorted_by_start[i][1]);
        i += 1;
      }

      // If the heap is empty, no events can be attended today
      // Skip to the next event start day
      if Solution::is_heap_empty(&end_day_heap) {
        if i < sorted_by_start.len() {
          today = sorted_by_start[i][0];
          continue;
        } else {
          break;
        }
      }

      // Trying to attend an event today, pick the one that ends the earliest
      // But it must not have already ended
      // Take from heap until encounter an end day >= today
      loop {
        if let Some(earliest_end_day) = Solution::extract_min_from_heap(&mut end_day_heap) {
          if earliest_end_day >= today {
            attended += 1;
            today += 1;
            break;
          }
        } else {
          break;
        }
      }
    }

    attended
  }

  // For simplicity, let's implement a heap starts at index 1
  pub fn left_node(i: usize) -> usize {
    i << 1
  }

  pub fn right_node(i: usize) -> usize {
    (i << 1) + 1
  }

  pub fn parent_node(i: usize) -> usize {
    i >> 1
  }

  pub fn min_heapify(data: &mut Vec<i32>, i: usize) {
    if i <= 0 || i >= data.len() {
      return;
    }

    let left = Self::left_node(i);
    let right = Self::right_node(i);

    let mut smallest = i;

    if left < data.len() && data[left] < data[smallest] {
      smallest = left;
    }

    if right < data.len() && data[right] < data[smallest] {
      smallest = right;
    }

    if smallest != i {
      data.swap(i, smallest);
      Self::min_heapify(data, smallest);
    }
  }

  pub fn build_min_heap(data: &mut Vec<i32>) {
    for i in (1..=(data.len() - 1) / 2).rev() {
      Self::min_heapify(data, i);
    }
  }

  pub fn extract_min_from_heap(data: &mut Vec<i32>) -> Option<i32> {
    if data.len() <= 1 {
      return None;
    }

    let min_value = data[1];
    let last_index = data.len() - 1;
    data[1] = data[last_index];
    data.pop();
    Self::min_heapify(data, 1);

    Some(min_value)
  }

  pub fn insert_into_heap(data: &mut Vec<i32>, value: i32) {
    data.push(value);
    let mut i = data.len() - 1;
    let mut parent = Self::parent_node(i);
    while i > 1 && data[parent] > data[i] {
      data.swap(i, parent);
      i = parent;
      parent = Self::parent_node(i);
    }
  }

  pub fn is_heap_empty(data: &Vec<i32>) -> bool {
    data.len() <= 1
  }

  // pub fn radix_sort_events(events: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  //   let sorted_by_start = Self::counting_sort_events(events, 0);
  //   sorted_by_start
  // }

  pub fn counting_sort_events(events: Vec<Vec<i32>>, sort_key: i32) -> Vec<Vec<i32>> {
    let mut count: Vec<i32> = vec![0; Self::MAX_EVENTS as usize + 1];

    for event in events.iter() {
      count[event[sort_key as usize] as usize] += 1;
    }

    for i in 1..count.len() {
      count[i] += count[i - 1];
    }

    let mut output: Vec<Vec<i32>> = vec![vec![0; 2]; events.len()];

    // Need to loop from the end of events to maintain stability
    for i in (0..events.len()).rev() {
      let event = &events[i];
      let key = event[sort_key as usize] as usize;
      output[count[key] as usize - 1] = event.clone();
      count[key] -= 1;
    }

    output
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_heap_nodes() {
    assert_eq!(Solution::left_node(1), 2);
    assert_eq!(Solution::right_node(1), 3);
    assert_eq!(Solution::parent_node(2), 1);
    assert_eq!(Solution::parent_node(3), 1);
    assert_eq!(Solution::parent_node(4), 2);
    assert_eq!(Solution::parent_node(5), 2);
  }

  #[test]
  fn test_heap() {
    let mut data: Vec<i32> = vec![0, 5, 3, 8, 4, 1, 7, 2, 6];
    Solution::build_min_heap(&mut data);
    assert_eq!(data, vec![0, 1, 3, 2, 4, 5, 7, 8, 6]);
  }

  #[test]
  fn test_counting_sort_events() {
    let events: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
    let sorted_by_start = Solution::counting_sort_events(events.clone(), 0);
    assert_eq!(sorted_by_start, vec![vec![1, 3], vec![2, 2], vec![3, 1]]);

    let sorted_by_end = Solution::counting_sort_events(events.clone(), 1);
    assert_eq!(sorted_by_end, vec![vec![3, 1], vec![2, 2], vec![1, 3]]);
  }

  // #[test]
  // fn test_radix_sort_events() {
  //   let events: Vec<Vec<i32>> = vec![vec![1, 4], vec![2, 2], vec![2, 3], vec![1, 3]];
  //   let sorted_events = Solution::radix_sort_events(events);
  //   assert_eq!(
  //     sorted_events,
  //     vec![vec![1, 4], vec![1, 3], vec![2, 2], vec![2, 3]]
  //   );
  // }

  #[test]
  fn test_max_events() {
    let events1: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 3], vec![2, 2], vec![1, 2]];
    assert_eq!(Solution::max_events(events1), 3);

    let events2: Vec<Vec<i32>> = vec![vec![52, 79], vec![7, 34]];
    assert_eq!(Solution::max_events(events2), 2);
  }
}
