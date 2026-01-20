pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Bottom-Up Solution

  // Call V[i] min number of coins can be used to exchange for i amount
  // V[0] = 0
  // V[i] = 1 if i in coins
  // V[i] = Min(V[i-c] + 1) for all coins c where c <= i

  // For example:
  // coins = [1, 3, 4]
  // amount = 6
  // V[0] = 0
  // V[1] = Min(V[0] + 1) = Min(1) = 1
  // V[2] = Min(V[1] + 1) = Min(2) = 2
  // V[3] = Min(V[2] + 1, V[0] + 1) = Min(3, 1) = 1
  // V[4] = Min(V[3] + 1, V[1] + 1, V[0] + 1) = Min(2, 2, 1) = 1
  // V[5] = Min(V[4] + 1, V[2] + 1, V[1] + 1) = Min(2, 3, 2) = 2
  // V[6] = Min(V[5] + 1, V[3] + 1, V[2] + 1) = Min(3, 2, 3) = 2

  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
      return 0;
    }

    let amt = amount as usize;
    let min_denom: usize;

    if let Some(&min) = coins.iter().min() {
      min_denom = min as usize;
    } else {
      return 0;
    }

    if amt < min_denom {
      return -1;
    }

    let coins_usize: Vec<usize> = coins.into_iter().map(|x| x as usize).collect();
    let mut v: Vec<i32> = vec![-1; amt + 1];

    v[0] = 0;

    for i in min_denom..=amt {
      for &c in coins_usize.iter() {
        if i < c {
          continue;
        }
        if i == c {
          v[i] = 1;
          continue;
        }
        let r = i - c;
        if v[r] < 0 {
          continue;
        }
        if v[i] < 0 {
          v[i] = v[r] + 1;
        } else {
          v[i] = v[i].min(v[r] + 1);
        }
      }
    }

    v[amt]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution() {
    // Example 1
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);

    // Example 2
    assert_eq!(Solution::coin_change(vec![2], 9), -1);

    // Example 3
    assert_eq!(Solution::coin_change(vec![1], 0), 0);

    // Additional test: Greedy fails
    assert_eq!(Solution::coin_change(vec![1, 3, 4], 6), 2);
  }
}
