pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Pad with 2 extra ballons with value 1 at both ends.
  // Let m[i][j] be the max coins that can be obtained by bursting all ballons in range [i+1..=j-1] (exclusive of i and j).

  // Instead of asking "Which balloon do I pop first?", ask: "Which balloon in this range [i+1..=j-1] is the absolute LAST one to be popped?"
  //   - Isolation: If balloon k is the last to pop in the range,
  //     then every other balloon in [i+1..=k-1], and every balloon in [k+1..=j-1], has already been popped.
  //   - Stable Neighbors: Because k is the last one standing in that range,
  //     its neighbors at the moment of popping are guaranteed to be i and j.

  // Therefore, if k is the last balloon to pop in range [i+1..=j-1],
  // the coins obtained from popping k is ballons[i] * ballons[k] * ballons[j].
  // The total coins obtained in this scenario is:
  //   m[i][k] + m[k][j] + ballons[i] * ballons[k] * ballons[j]

  // Recurrence Relation:
  //   m[i][j] = Max(
  //     m[i][k] + m[k][j] + ballons[i] * ballons[k] * ballons[j]
  //   ) for all k in [i+1..j-1]
  // Base Case:
  //   m[i-1][i+1] = ballons[i-1] * ballons[i] * ballons[i+1] for all i in [1..=n]

  const N: usize = 302;

  pub fn max_coins(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
      return nums[0];
    }

    let mut ballons = vec![1];
    ballons.extend(nums);
    ballons.push(1);

    let mut m: [[i32; Self::N]; Self::N] = [[0; Self::N]; Self::N];

    for i in 1..=n {
      m[i - 1][i + 1] = ballons[i - 1] * ballons[i] * ballons[i + 1];
    }

    for l in 2..=n {
      for i in 0..=(n - l) {
        let j = i + l + 1;
        for k in (i + 1)..j {
          m[i][j] = m[i][j].max(m[i][k] + m[k][j] + ballons[i] * ballons[k] * ballons[j]);
        }
      }
    }

    m[0][n + 1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    assert_eq!(Solution::max_coins(vec![3, 5, 8]), 152);
    assert_eq!(Solution::max_coins(vec![3, 5]), 20);
    assert_eq!(Solution::max_coins(vec![5]), 5);
  }
}
