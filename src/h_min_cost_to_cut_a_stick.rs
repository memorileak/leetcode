pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // This problem can be translated and understood in a different way:
  // Given a list of k cuts to be made on a stick of length n,
  // When all k cuts are made, the stick is finally divided into k+1 segments.
  // The cost of cutting a long segment into two sub-segments
  // is equal to the cost of joining the two sub-segments back together.
  // So, the problem becomes:
  // Given k+1 segments, we need to find the order of joining them back together
  // such that the total cost is minimized.
  // This is very similar to the matrix chain multiplication problem,
  // which can be solved using dynamic programming.
  //
  // Let c[i][j] be the minimum cost to join segments from i to j.
  // Then, for each possible split point k between i and j,
  // we can calculate the cost of joining segments from i to k and k+1 to j,
  // plus the cost of joining the two resulting segments together.
  // Thus, the recurrence relation is:
  // c[i][j] = min(c[i][k] + c[k+1][j] + cost(i, j)) for all k in [i+1, j-1]
  // where cost(i, j) is the total length of segments from i to j.
  // Base case: if i >= j - 1, then c[i][j] = 0.
  pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
    let mut all_sorted_cuts = cuts;
    all_sorted_cuts.push(0);
    all_sorted_cuts.push(n);
    all_sorted_cuts.sort_unstable();

    let m = all_sorted_cuts.len();
    let mut c = vec![vec![0; m]; m];

    for j in 2..m {
      for i in 0..j - 1 {
        Self::calc_cost(&all_sorted_cuts, &mut c, i, j);
      }
    }

    c[0][m - 1]
  }

  pub fn calc_cost(all_sorted_cuts: &Vec<i32>, c: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i >= j - 1 {
      c[i][j] = 0;
      return c[i][j];
    }

    if c[i][j] != 0 {
      return c[i][j];
    }

    let mut min_cost = i32::MAX;
    let join_cost = all_sorted_cuts[j] - all_sorted_cuts[i];
    for k in (i + 1)..j {
      let left_cost = Solution::calc_cost(all_sorted_cuts, c, i, k);
      let right_cost = Solution::calc_cost(all_sorted_cuts, c, k, j);
      min_cost = min_cost.min(left_cost + right_cost + join_cost);
    }
    c[i][j] = min_cost;
    return min_cost;
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_min_cost() {
    let n = 7;
    let cuts = vec![1, 3, 4, 5];
    let result = Solution::min_cost(n, cuts);
    assert_eq!(result, 16);

    let n2 = 9;
    let cuts2 = vec![5, 6, 1, 4, 2];
    let result2 = Solution::min_cost(n2, cuts2);
    assert_eq!(result2, 22);
  }
}
