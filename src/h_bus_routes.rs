use std::collections::{HashSet, VecDeque};

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target {
      return 0;
    }

    // Bus route i goes through stations routes[i] = [s0, s1, ..., sn]
    // Now given a station s, determine all the buses (bus routes) that go through it.
    // We say 2 bus routes are connected if they both go through at least 1 common station.
    // When 2 bus routes are connected, you can switch from one route to another.

    // Maximum number of routes: 500
    // Maximum number of stations: 10^6
    // Each route goes through at most 10^5 unique stations.

    // Mapping a station to every bus routes go through it
    let mut sr: Vec<Option<HashSet<usize>>> = vec![None; 1_000_001];

    for (r, stations) in routes.iter().enumerate() {
      for s in stations.iter() {
        let s_usize = *s as usize;
        if let Some(set_of_routes) = sr[s_usize].as_mut() {
          set_of_routes.insert(r);
        } else {
          sr[s_usize] = Some(HashSet::from([r]));
        }
      }
    }

    // Routes could be used to start the journey
    let start_routes: &HashSet<usize>;
    if let Some(routes) = sr[source as usize].as_ref() {
      start_routes = routes;
    } else {
      return -1;
    }

    // Routes could be used to reach the target
    let end_routes: &HashSet<usize>;
    if let Some(routes) = sr[target as usize].as_ref() {
      end_routes = routes;
    } else {
      return -1;
    }

    // println!("Start: {:?}", start_routes);
    // println!("End: {:?}", end_routes);

    // Early terminate: one of start_routes presents in end_routes
    for start in start_routes.iter() {
      if end_routes.contains(start) {
        return 1;
      }
    }

    // Mapping a route to every sets of routes which contain it
    let mut pre_rr: Vec<Vec<&HashSet<usize>>> = vec![vec![]; 501];
    for set_of_routes_opt in sr.iter() {
      if let Some(set_of_routes) = set_of_routes_opt.as_ref() {
        for &r in set_of_routes.iter() {
          pre_rr[r].push(set_of_routes);
        }
      }
    }

    // Build the graph: mapping a route to every other routes connected to it.
    let mut rr: Vec<HashSet<usize>> = vec![HashSet::new(); 501];
    for (r, sets_of_connected_routes) in pre_rr.iter().enumerate() {
      if sets_of_connected_routes.len() > 0 {
        for set_of_routes in sets_of_connected_routes.iter() {
          rr[r].extend(set_of_routes.iter().filter(|v| (**v) != r).cloned())
        }
      }
    }

    // for i in 0..10 {
    //   println!("{}: {:?}", i, rr[i]);
    // }

    // Use BFS to find the shortest path from start_routes to end_routes
    // Concurrently start from every routes in start_routes,
    // End the search when reaching any route in end_routes.
    let mut queue: VecDeque<(i32, usize)> =
      VecDeque::from_iter(start_routes.iter().map(|r| (1, *r)));
    let mut visited: HashSet<usize> = HashSet::from_iter(start_routes.iter().cloned());

    while !queue.is_empty() {
      let (level, route) = queue.pop_front().unwrap();
      for nei_route in rr[route].iter() {
        if !visited.contains(nei_route) {
          if end_routes.contains(nei_route) {
            return level + 1;
          }
          queue.push_back((level + 1, *nei_route));
          visited.insert(*nei_route);
        }
      }
    }

    -1
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_two_buses_needed() {
    // Take bus 1 (stops 1,2,7), then bus 2 (stops 3,6,7) to reach 6
    assert_eq!(
      Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6),
      2
    );
  }

  #[test]
  fn test_no_route_exists() {
    // No combination of buses can reach stop 12 from stop 15
    assert_eq!(
      Solution::num_buses_to_destination(
        vec![
          vec![7, 12],
          vec![4, 5, 15],
          vec![6],
          vec![15, 19],
          vec![9, 12, 13]
        ],
        15,
        12
      ),
      -1
    );
  }

  #[test]
  fn test_source_equals_target() {
    // Already at destination, no buses needed
    assert_eq!(
      Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 1),
      0
    );
  }

  #[test]
  fn test_one_bus_needed() {
    // Source and target are on the same bus route
    assert_eq!(
      Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 7),
      1
    );
  }
}
