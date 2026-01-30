pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Let m[i][j] be the minimum cost to build a tree from leaf values arr[i..=j].
  // Base case:
  //   m[i][i] = 0 for all i in [0..=n-1] (as single leaf node has no non-leaf parent node, hence no cost)
  // Recurrence Relation:
  //   m[i][j] = Min(
  //     m[i][k] + m[k+1][j] + cost(i, k, j)
  //   ) for all k in [i..=j-1]
  // Where: cost(i, k, j) = maxleaf(i, k) * maxleaf(k+1, j)
  // With maxleaf(i, k) being the maximum leaf value in arr[i..=k].
  // We can precompute maxleaf(i, j) for all i, j in [0..=n-1] also using dynamic programming:
  //   maxleaf[i][i] = arr[i] for all i in [0..=n-1]
  //   maxleaf[i][j] = max(maxleaf[i][j-1], arr[j]) for all i in [0..=n-2], j in [i+1..=n-1]

  const N: usize = 40;

  pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut maxleaf: [[i32; Self::N]; Self::N] = [[0; Self::N]; Self::N];

    for i in 0..n {
      maxleaf[i][i] = arr[i];
    }

    for i in 0..(n - 1) {
      for j in (i + 1)..n {
        maxleaf[i][j] = maxleaf[i][j - 1].max(arr[j]);
      }
    }

    let mut m: [[i32; Self::N]; Self::N] = [[0; Self::N]; Self::N];

    for l in 2..=n {
      for i in 0..=(n - l) {
        let j = i + l - 1;
        m[i][j] = m[i][i] + m[i + 1][j] + maxleaf[i][i] * maxleaf[i + 1][j];
        for k in (i + 1)..=(j - 1) {
          m[i][j] = m[i][j].min(m[i][k] + m[k + 1][j] + maxleaf[i][k] * maxleaf[k + 1][j]);
        }
      }
    }

    m[0][n - 1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
    assert_eq!(Solution::mct_from_leaf_values(vec![4, 11]), 44);
  }
}
