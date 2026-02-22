use std::collections::VecDeque;

pub struct St(u8, u16, u16); // State(node, traversed_path_length, visited_nodes_bitmask)

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // "Shortest Path" in an unweighted graph (where every edge costs 1 step) -> BFS.
  // Cause BFS explores all path of length 1, then 2, then 3, etc,
  // it guarantees the first time we reach the target, we have found the shortest path.
  //
  // But, this problem is not so straight forward: we may revisit nodes and reuse edges.
  // In a star graph where 0 connects to 1, 2, 3; we have to go 1 -> 0 -> 2 -> 0 -> 3.
  // The node 0 is visited twice.
  // If we just track that "node 0 is visited" and don't visit it again, we will never reach the target.
  //
  // Observe that the first time we visit node 0, the visited nodes are {0, 1}
  // The second time we revisit node 0, the visited nodes are {0, 1, 2}.
  // So, if a node is visited again, but with a larger set of visited nodes,
  // it is a productive revisit, and we should explore it again.
  //
  // So, the correct tracking of visited states is:
  // "node 0 is visited with visited nodes {0, 1}"
  // "node 0 is visited with visited nodes {0, 1, 2}"
  // They are two different states.
  // A set of visited nodes is attached to the node.
  //
  // In this problem, the number of nodes in the graph is at most 12
  // We can utilize a bitmask to represent the set of visited nodes efficiently.
  // If node i is visited, bit at position i is 1, otherwise it is 0.
  // For example, if we have 4 nodes and we have visited nodes 0, 2, the bitmask is 1010.
  //
  // We can use a HashSet<u16> to track that state.
  // In each u16: first 4 bits are the node, last 12 bits are the visited-nodes-bitmask.
  //
  // Or alternatively, we can use a Vec<u16> of size 2^n.
  // The index of the Vec is the visited-nodes-bitmask,
  // Each element in the Vec holds a 16 bit integer:
  // If the i-th bit of that integer is 1, it means we have explored this mask while standing on node i.
  //
  // The problem says we can start at any node.
  // Running n separate BFS algorithms is not a good move.
  // Instead, we can initialize the BFS queue with every single node as a starting point simultaneously,
  // treating them all as distance 0.
  // The BFS will expand them all fairly, one step at a time.
  // The first one to cross the finish line (visited all nodes) wins.
  pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    let n: u16 = graph.len() as u16;
    if n == 1 {
      return 0;
    }

    let t: u16 = (1 << n) - 1;

    let mut queue: VecDeque<St> = VecDeque::new();
    let mut seen: Vec<u16> = vec![0; t as usize + 1];

    for i in 0..n {
      let mask: u16 = 1_u16 << (i as u16);
      queue.push_back(St(i as u8, 0, mask));
      seen[mask as usize] = mask;
    }

    while !queue.is_empty() {
      let St(node, path_len, mask) = queue.pop_front().unwrap();
      let neighbors = &graph[node as usize];
      for &i in neighbors.iter() {
        let i_u8 = i as u8;
        let i_u16 = i as u16;
        let mask_i = mask | (1_u16 << i_u16);

        if (seen[mask_i as usize] & (1_u16 << i_u16)) == 0 {
          if mask_i == t {
            return path_len as i32 + 1;
          }
          seen[mask_i as usize] |= 1_u16 << i_u16;
          queue.push_back(St(i_u8, path_len + 1, mask_i));
        }
      }
    }

    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_shortest_path_length_example1() {
    let graph = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
    assert_eq!(Solution::shortest_path_length(graph), 4);
  }

  #[test]
  fn test_shortest_path_length_example2() {
    let graph = vec![vec![1], vec![0, 2, 4], vec![1, 3, 4], vec![2], vec![1, 2]];
    assert_eq!(Solution::shortest_path_length(graph), 4);
  }

  #[test]
  fn test_shortest_path_length_example3() {
    let graph: Vec<Vec<i32>> = [
      vec![3, 8],
      vec![2, 6, 7],
      vec![1, 6],
      vec![0, 5],
      vec![7],
      vec![3, 8],
      vec![1, 2],
      vec![1, 4, 8],
      vec![0, 5, 7],
    ]
    .to_vec();
    assert_eq!(Solution::shortest_path_length(graph), 9);
  }

  #[test]
  fn test_shortest_path_length_example4() {
    let graph: Vec<Vec<i32>> = [
      vec![7],
      vec![3],
      vec![3, 9],
      vec![1, 2, 4, 5, 7, 11],
      vec![3],
      vec![3],
      vec![9],
      vec![3, 10, 8, 0],
      vec![7],
      vec![11, 6, 2],
      vec![7],
      vec![3, 9],
    ]
    .to_vec();
    assert_eq!(Solution::shortest_path_length(graph), 17);
  }
}
