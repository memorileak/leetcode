pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;

    let m = grid.len();
    if m == 0 {
      return 0;
    }

    let n = grid[0].len();
    if n == 0 {
      return 0;
    }

    let mut max_area: i32 = 0;
    for i in 0..m {
      for j in 0..n {
        if grid[i][j] == 1 {
          max_area = max_area.max(Self::dfs_area(&mut grid, i, j))
        }
      }
    }

    max_area
  }

  fn dfs_area(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut stack: Vec<(usize, usize)> = vec![(i, j)];
    let mut area: i32 = 0;

    while !stack.is_empty() {
      let (i, j) = stack.pop().unwrap();
      if grid[i][j] == 1 {
        // DFS: Mark visited at pop-from-stack time
        // BFS: Mark visited at push-to-queue time
        grid[i][j] = 0; // Mark as visited
        area += 1; // Process the node

        // Left neighbor
        if j > 0 && grid[i][j - 1] == 1 {
          stack.push((i, j - 1));
        }

        // Right neighbor
        if j < n - 1 && grid[i][j + 1] == 1 {
          stack.push((i, j + 1));
        }

        // Above neighbor
        if i > 0 && grid[i - 1][j] == 1 {
          stack.push((i - 1, j));
        }

        // Below neighbor
        if i < m - 1 && grid[i + 1][j] == 1 {
          stack.push((i + 1, j));
        }
      }
    }

    area
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_basic_single_island() {
    let grid = vec![
      vec![0, 0, 0, 0, 0],
      vec![0, 1, 1, 0, 0],
      vec![0, 1, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::max_area_of_island(grid), 3);
  }

  #[test]
  fn test_multiple_islands() {
    let grid = vec![
      vec![0, 0, 1, 0, 0],
      vec![0, 1, 1, 0, 0],
      vec![0, 0, 0, 1, 1],
    ];
    assert_eq!(Solution::max_area_of_island(grid), 3);
  }

  #[test]
  fn test_no_islands() {
    let grid = vec![vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(Solution::max_area_of_island(grid), 0);
  }

  #[test]
  fn test_entire_grid_is_island() {
    let grid = vec![vec![1, 1], vec![1, 1]];
    assert_eq!(Solution::max_area_of_island(grid), 4);
  }

  #[test]
  fn test_single_cell_island() {
    let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(Solution::max_area_of_island(grid), 1);
  }

  #[test]
  fn test_diagonal_not_connected() {
    // Diagonal cells should NOT count as connected
    let grid = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    assert_eq!(Solution::max_area_of_island(grid), 1);
  }
}
