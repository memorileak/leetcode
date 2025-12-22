pub struct Solution;
pub struct SolutionUnionFind;

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

#[allow(dead_code)]
impl SolutionUnionFind {
  pub const MAX_EVENTS: i32 = 100_000;

  // Sort all events by end day, from earliest to latest.
  // For multiple events with the same end day, sort by start day from earliest to latest.
  // Loop through all events in the sorted order.
  // For each event, for each day from start day to end day, collect those days to a disjoint sets union called "available days".
  // At start, all days are available, so each day d points to d itself (those days are not connected).
  // For each event, find the smallest available day d in the "available days" that d >= start and d <= end using union-find.
  // If found, attend the event on that day, and remove that day from the "available days" by linking d to d+1.
  pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
    let sorted_by_end = Self::counting_sort_events(events, 1);

    let mut parent: Vec<usize> = vec![0; (Self::MAX_EVENTS + 2) as usize];

    for d in 1..=(Self::MAX_EVENTS + 1) as usize {
      Self::make_set(&mut parent, d);
    }

    let mut attended: i32 = 0;
    for event in sorted_by_end.iter() {
      let s = event[0] as usize;
      let e = event[1] as usize;
      let available_day = Self::find_set(&mut parent, s);

      if available_day <= e {
        attended += 1;
        Self::union(&mut parent, available_day, available_day + 1);
      }
    }

    attended
  }

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

  // Union-Find helper functions
  // Each node x has its parent[x]
  // Maximum value of x is MAX_EVENTS
  // Don't use rank[x] heuristic for this problem cause we always want to link x to x+1
  pub fn make_set(parent: &mut Vec<usize>, x: usize) {
    parent[x] = x;
  }

  pub fn union(parent: &mut Vec<usize>, x: usize, y: usize) {
    let rx = Self::find_set(parent, x);
    let ry = Self::find_set(parent, y);
    if rx != ry {
      Self::link(parent, rx, ry);
    }
  }

  pub fn link(parent: &mut Vec<usize>, rx: usize, ry: usize) {
    parent[rx] = ry;
  }

  pub fn find_set(parent: &mut Vec<usize>, x: usize) -> usize {
    let mut i = x;
    let mut traversed: Vec<usize> = vec![];
    while i != parent[i] {
      traversed.push(i);
      i = parent[i];
    }
    // Path compression
    for t in traversed.iter() {
      parent[*t] = i;
    }
    i
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;
  use super::SolutionUnionFind;

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

  #[test]
  fn test_union_find() {
    let mut parent: Vec<usize> = vec![0; 10];

    // Test make_set
    for x in 0..10 {
      SolutionUnionFind::make_set(&mut parent, x);
    }
    assert_eq!(parent, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // Test union and find_set
    // Union 1 and 2: 1 -> 2
    // Union 3 and 4: 3 -> 4
    // Union 1 and 3: 1 -> 2 -> 4 <- 3
    SolutionUnionFind::union(&mut parent, 1, 2);
    SolutionUnionFind::union(&mut parent, 3, 4);
    SolutionUnionFind::union(&mut parent, 1, 3);
    let root_1 = SolutionUnionFind::find_set(&mut parent, 1);
    let root_2 = SolutionUnionFind::find_set(&mut parent, 2);
    let root_3 = SolutionUnionFind::find_set(&mut parent, 3);
    let root_4 = SolutionUnionFind::find_set(&mut parent, 4);
    assert_eq!(root_1, root_2);
    assert_eq!(root_3, root_4);
    assert_eq!(root_1, root_3);
    assert_eq!(parent, vec![0, 4, 4, 4, 4, 5, 6, 7, 8, 9]);
  }

  #[test]
  fn test_union_find_max_events() {
    let events1: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 3], vec![2, 2], vec![1, 2]];
    assert_eq!(SolutionUnionFind::max_events(events1), 3);

    let events2: Vec<Vec<i32>> = vec![vec![52, 79], vec![7, 34]];
    assert_eq!(SolutionUnionFind::max_events(events2), 2);
  }
}
