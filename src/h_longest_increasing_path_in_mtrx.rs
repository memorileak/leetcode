pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    if m == 0 {
      return 0;
    }

    let n = matrix[0].len();
    if n == 0 {
      return 0;
    }

    let matrix: Vec<i32> = matrix.into_iter().flatten().collect();

    // The visited set is for preventing cycles.
    // But in this problem, the path is strictly increasing, making the whole graph a DAG.
    // There won't be any cycles here, so DFS can be done without visited set.

    // Optimized this way but the algorithm is still slow.
    // Could you give some idea to improve the runtime AI?
    let mut stack: Vec<(u16, usize)> = vec![];
    let mut parent: Vec<Option<usize>> = vec![None; m * n];
    let mut longest: Vec<u16> = vec![0; m * n];
    let mut final_longest: u16 = 0;

    for src in 0..(m * n) {
      if longest[src] > 0 {
        // The longest path from source src has been calculated before, just skip
        continue;
      }

      stack.push((1, src));

      while !stack.is_empty() {
        let (l, i) = stack.pop().unwrap();

        // Process node: just print it out
        // for _ in 1..l as usize {
        //   print!("|--");
        // }
        // println!("{}", matrix[i]);

        if longest[i] > 0 {
          // The longest path from source i has been calculated before
          // Just trace back from i to the start
          let mut ci = i;
          while let Some(pci) = parent[ci] {
            longest[pci] = longest[pci].max(longest[ci] + 1);
            final_longest = final_longest.max(longest[pci]);
            ci = pci;
          }
        } else {
          let mut is_dead_end = true;

          // Left neighbor
          if i % n > 0 && matrix[i - 1] > matrix[i] {
            stack.push((l + 1, i - 1));
            parent[i - 1] = Some(i);
            is_dead_end = false;
          }

          // Right neighbor
          if i % n < n - 1 && matrix[i + 1] > matrix[i] {
            stack.push((l + 1, i + 1));
            parent[i + 1] = Some(i);
            is_dead_end = false;
          }

          // Above neighbor
          if i >= n && matrix[i - n] > matrix[i] {
            stack.push((l + 1, i - n));
            parent[i - n] = Some(i);
            is_dead_end = false;
          }

          // Below neighbor
          if i < (m - 1) * n && matrix[i + n] > matrix[i] {
            stack.push((l + 1, i + n));
            parent[i + n] = Some(i);
            is_dead_end = false;
          }

          // Dead end reached, trace back the path
          if is_dead_end == true {
            let mut ci = i;
            longest[ci] = 1;
            final_longest = final_longest.max(longest[ci]);
            while let Some(pci) = parent[ci] {
              longest[pci] = longest[pci].max(longest[ci] + 1);
              final_longest = final_longest.max(longest[pci]);
              ci = pci;
            }
          }
        }
      }

      // Cleanup
      stack.clear();
      parent.fill(None);
    }

    // for i in 0..m {
    //   for j in 0..n {
    //     let idx = i * n + j;
    //     print!("{} ", longest[idx]);
    //   }
    //   println!();
    // }

    final_longest as i32
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_example_1() {
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
  }

  #[test]
  fn test_example_2() {
    let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
  }

  #[test]
  fn test_example_3() {
    let matrix = vec![
      vec![13, 5, 13, 9],
      vec![5, 0, 2, 9],
      vec![10, 13, 11, 10],
      vec![0, 0, 13, 13],
    ];
    assert_eq!(Solution::longest_increasing_path(matrix), 6);
  }

  #[test]
  fn test_single_cell() {
    let matrix = vec![vec![1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 1);
  }

  #[test]
  fn test_all_same_values() {
    let matrix = vec![vec![1, 1], vec![1, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 1);
  }

  #[test]
  fn test_single_row() {
    let matrix = vec![vec![1, 2, 3, 4]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
  }

  #[test]
  fn test_single_column() {
    let matrix = vec![vec![4], vec![3], vec![2], vec![1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
  }
}
