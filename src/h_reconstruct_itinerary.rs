// Hierholzer's Algorithm — Pseudocode
//
// ## Purpose
//
// Find an Eulerian circuit (or Eulerian path) in a graph where one exists.
//
// --------
//
//   procedure Hierholzer(G):
//
//       // If finding an Euler PATH (not circuit),
//       // set start_vertex = a vertex with odd degree
//       // Otherwise, start_vertex = any vertex with edges
//
//       Initialize an empty stack: S
//       Initialize an empty list:  circuit
//
//       Push start_vertex onto S
//
//       while S is not empty:
//           v = top of S (peek)
//
//           if v has any remaining (unvisited) edges:
//               u = any adjacent vertex with an unvisited edge (v, u)
//               mark edge (v, u) as visited (remove it)
//               push u onto S
//           else:
//               pop v from S
//               append v to circuit
//
//       // 'circuit' now contains the Eulerian circuit/path
//       // (vertices in reverse order of traversal, or reverse it if needed)
//
//       return circuit
//
// --------
//
// ## Key Ideas
//
//  Step      │ What happens
// ───────────┼──────────────────────────────────────────────────────────────
//  Traverse  │ Greedily follow unvisited edges, pushing vertices onto the
//            │ stack.
//  Backtrack │ When a vertex has no remaining edges, pop it and record it
//            │ in the result.
//  Stitch    │ The stack naturally "stitches" sub-tours together — this is
//            │ the core insight of Hierholzer's algorithm.
//
// ## Preconditions
//
// • Eulerian Circuit: Every vertex has even degree (undirected) or equal in-
// degree and out-degree (directed).
// • Eulerian Path: Exactly two vertices have odd degree (undirected) or
// exactly one vertex has  out−in = 1  and one has  in−out = 1  (directed).
// Start from the appropriate vertex.
//
// ## Complexity
//
// • Time: O(E)
// • Space: O(E)
use std::collections::{HashMap, HashSet};

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for t in tickets.iter() {
      graph.entry(&t[0]).or_default().push(&t[1]);
    }

    for (_, v) in graph.iter_mut() {
      v.sort_unstable();
    }

    let mut stack: Vec<&str> = Vec::new();
    let mut path: Vec<&str> = Vec::new();
    let mut used_edges: HashSet<(&str, usize)> = HashSet::new();
    let no_neighbors: Vec<&str> = vec![];

    stack.push("JFK");

    while !stack.is_empty() {
      let &node = stack.last().unwrap();

      let neighbors = graph.get(node).unwrap_or(&no_neighbors);
      let mut has_undiscovered_edges = false;

      for i in 0..neighbors.len() {
        let edge = (node, i);
        if !used_edges.contains(&edge) {
          stack.push(neighbors[i]);
          used_edges.insert(edge);
          has_undiscovered_edges = true;
          break;
        }
      }

      if has_undiscovered_edges == false {
        path.push(stack.pop().unwrap());
      }
    }

    path.into_iter().rev().map(|s| String::from(s)).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_example_1() {
    let tickets = vec![
      vec!["MUC".to_string(), "LHR".to_string()],
      vec!["JFK".to_string(), "MUC".to_string()],
      vec!["SFO".to_string(), "SJC".to_string()],
      vec!["LHR".to_string(), "SFO".to_string()],
    ];
    assert_eq!(
      Solution::find_itinerary(tickets),
      vec![
        "JFK".to_string(),
        "MUC".to_string(),
        "LHR".to_string(),
        "SFO".to_string(),
        "SJC".to_string(),
      ]
    );
  }

  #[test]
  fn test_example_2() {
    let tickets = vec![
      vec!["JFK".to_string(), "SFO".to_string()],
      vec!["JFK".to_string(), "ATL".to_string()],
      vec!["SFO".to_string(), "ATL".to_string()],
      vec!["ATL".to_string(), "JFK".to_string()],
      vec!["ATL".to_string(), "SFO".to_string()],
    ];
    assert_eq!(
      Solution::find_itinerary(tickets),
      vec![
        "JFK".to_string(),
        "ATL".to_string(),
        "JFK".to_string(),
        "SFO".to_string(),
        "ATL".to_string(),
        "SFO".to_string()
      ]
    );
  }

  #[test]
  fn test_example_3() {
    let tickets: Vec<Vec<String>> = vec![
      vec!["EZE", "AXA"],
      vec!["TIA", "ANU"],
      vec!["ANU", "JFK"],
      vec!["JFK", "ANU"],
      vec!["ANU", "EZE"],
      vec!["TIA", "ANU"],
      vec!["AXA", "TIA"],
      vec!["TIA", "JFK"],
      vec!["ANU", "TIA"],
      vec!["JFK", "TIA"],
    ]
    .into_iter()
    .map(|v| vec![v[0].to_string(), v[1].to_string()])
    .collect();
    assert_eq!(
      Solution::find_itinerary(tickets),
      vec![
        "JFK", "ANU", "EZE", "AXA", "TIA", "ANU", "JFK", "TIA", "ANU", "TIA", "JFK",
      ]
      .into_iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
    )
  }
}
