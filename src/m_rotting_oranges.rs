pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;

    let m = grid.len();
    if m == 0 {
      return 0;
    }

    let n = grid[0].len();
    if n == 0 {
      return 0;
    }

    let mut minutes: i32 = 0;

    // BFS simultaneously from all 2 nodes
    let mut queue: std::collections::VecDeque<(usize, usize, usize)> =
      std::collections::VecDeque::new();
    let mut fresh: usize = 0;
    for i in 0..m {
      for j in 0..n {
        if grid[i][j] == 2 {
          queue.push_back((0, i, j));
        } else if grid[i][j] == 1 {
          fresh += 1;
        }
      }
    }

    while !queue.is_empty() {
      let (l, i, j) = queue.pop_front().unwrap();
      minutes = l as i32;

      // Left neighbor
      if j > 0 && grid[i][j - 1] == 1 {
        grid[i][j - 1] = 2;
        queue.push_back((l + 1, i, j - 1));
        fresh -= 1;
      }

      // Right neighbor
      if j < n - 1 && grid[i][j + 1] == 1 {
        grid[i][j + 1] = 2;
        queue.push_back((l + 1, i, j + 1));
        fresh -= 1;
      }

      // Above neighbor
      if i > 0 && grid[i - 1][j] == 1 {
        grid[i - 1][j] = 2;
        queue.push_back((l + 1, i - 1, j));
        fresh -= 1;
      }

      // Below neighbor
      if i < m - 1 && grid[i + 1][j] == 1 {
        grid[i + 1][j] = 2;
        queue.push_back((l + 1, i + 1, j));
        fresh -= 1;
      }
    }

    if fresh > 0 {
      // There are isolated fresh oranges
      minutes = -1;
    }

    minutes
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_example_1() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    assert_eq!(Solution::oranges_rotting(grid), 4);
  }

  #[test]
  fn test_example_2() {
    let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
    assert_eq!(Solution::oranges_rotting(grid), -1);
  }

  #[test]
  fn test_example_3() {
    let grid = vec![vec![0, 2]];
    assert_eq!(Solution::oranges_rotting(grid), 0);
  }
}
