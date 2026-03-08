pub struct Solution;

// ## Iterative BFS and DFS Pseudocode
//
// ### BFS (Breadth-First Search) - Uses Queue
//
//   BFS(graph, start):
//       queue = [start]
//       visited = {start}
//
//       while queue is not empty:
//           node = queue.dequeue()        # remove from FRONT
//           process(node)
//
//           for each neighbor in graph[node]:
//               if neighbor not in visited:
//                   visited.add(neighbor)
//                   queue.enqueue(neighbor)  # add to BACK
//
// ### DFS (Depth-First Search) - Uses Stack
//
//   DFS(graph, start):
//       stack = [start]
//       visited = {}
//
//       while stack is not empty:
//           node = stack.pop()            # remove from TOP
//
//           if node not in visited:
//               visited.add(node)
//               process(node)
//
//               for each neighbor in graph[node]:
//                   if neighbor not in visited:
//                       stack.push(neighbor)  # add to TOP
//
// ### Key Differences
//
//                         │ BFS                    │ DFS
// ────────────────────────┼────────────────────────┼────────────────────────
//  Data Structure         │ Queue (FIFO)           │ Stack (LIFO)
//  Order                  │ Level by level         │ Depth first
//  Visited check          │ Before enqueue         │ After pop
//
// │ Note: In BFS, mark visited before adding to queue to avoid duplicates. In
// │ iterative DFS, check visited after popping.

#[allow(dead_code)]
impl Solution {
  pub fn solve(board: &mut Vec<Vec<char>>) {
    let m = board.len();
    if m == 0 {
      return;
    }

    let n = board[0].len();
    if n == 0 {
      return;
    }

    for j in 0..n {
      if board[0][j] == 'O' {
        Self::dfs_flood(board, 0, j);
      }
      if board[m - 1][j] == 'O' {
        Self::dfs_flood(board, m - 1, j);
      }
    }

    for i in 1..(m - 1) {
      if board[i][0] == 'O' {
        Self::dfs_flood(board, i, 0);
      }
      if board[i][n - 1] == 'O' {
        Self::dfs_flood(board, i, n - 1);
      }
    }

    for i in 0..m {
      for j in 0..n {
        if board[i][j] == 'S' {
          board[i][j] = 'O';
        } else if board[i][j] == 'O' {
          board[i][j] = 'X';
        }
      }
    }
  }

  fn dfs_flood(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let m = board.len();
    let n = board[0].len();
    let mut stack: Vec<(usize, usize)> = vec![(i, j)];

    while !stack.is_empty() {
      let (i, j) = stack.pop().unwrap();
      if board[i][j] == 'O' {
        board[i][j] = 'S';

        // Left neighbor
        if j > 0 && board[i][j - 1] == 'O' {
          stack.push((i, j - 1));
        }

        // Right neighbor
        if j < n - 1 && board[i][j + 1] == 'O' {
          stack.push((i, j + 1));
        }

        // Above neighbor
        if i > 0 && board[i - 1][j] == 'O' {
          stack.push((i - 1, j));
        }

        // Below neighbor
        if i < m - 1 && board[i + 1][j] == 'O' {
          stack.push((i + 1, j));
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn to_board(rows: Vec<&str>) -> Vec<Vec<char>> {
    rows.iter().map(|r| r.chars().collect()).collect()
  }

  #[test]
  fn test_example_1() {
    let mut board = to_board(vec!["XXXX", "XOOX", "XXOX", "XOXX"]);
    let expected = to_board(vec!["XXXX", "XXXX", "XXXX", "XOXX"]);
    Solution::solve(&mut board);
    assert_eq!(board, expected);
  }

  #[test]
  fn test_example_2() {
    let mut board = to_board(vec!["X"]);
    let expected = to_board(vec!["X"]);
    Solution::solve(&mut board);
    assert_eq!(board, expected);
  }

  #[test]
  fn test_single_o() {
    let mut board = to_board(vec!["O"]);
    let expected = to_board(vec!["O"]);
    Solution::solve(&mut board);
    assert_eq!(board, expected);
  }

  #[test]
  fn test_border_o_not_flipped() {
    // O's on the border should never be flipped
    let mut board = to_board(vec!["OXO", "XOX", "OXO"]);
    let expected = to_board(vec!["OXO", "XXX", "OXO"]);
    Solution::solve(&mut board);
    assert_eq!(board, expected);
  }

  #[test]
  fn test_all_x() {
    let mut board = to_board(vec!["XXX", "XXX", "XXX"]);
    let expected = to_board(vec!["XXX", "XXX", "XXX"]);
    Solution::solve(&mut board);
    assert_eq!(board, expected);
  }

  #[test]
  fn test_all_o() {
    // All O's are connected to the border, none should be flipped
    let mut board = to_board(vec!["OOO", "OOO", "OOO"]);
    let expected = to_board(vec!["OOO", "OOO", "OOO"]);
    Solution::solve(&mut board);
    assert_eq!(board, expected);
  }

  #[test]
  fn test_surrounded_region_flipped() {
    // Inner O fully surrounded by X's should be flipped
    let mut board = to_board(vec!["XXXXX", "XOOOX", "XOXOX", "XOOOX", "XXXXX"]);
    let expected = to_board(vec!["XXXXX", "XXXXX", "XXXXX", "XXXXX", "XXXXX"]);
    Solution::solve(&mut board);
    assert_eq!(board, expected);
  }
}
