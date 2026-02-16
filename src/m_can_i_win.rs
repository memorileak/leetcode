pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if desired_total <= 0 {
      return true;
    }

    let max_possible_sum = max_choosable_integer * (max_choosable_integer + 1) / 2;
    if max_possible_sum < desired_total {
      return false;
    }

    let choosable: usize = max_choosable_integer as usize;
    let desired: usize = desired_total as usize;
    let mut computed: Vec<Option<bool>> = vec![None; 1 << 20];
    Self::solve(choosable, desired, &mut computed, 0, 0)
  }

  pub fn solve(
    choosable: usize,
    desired: usize,
    computed: &mut Vec<Option<bool>>,
    bitmask: usize,
    current_sum: usize,
  ) -> bool {
    // for i in 1..=choosable {
    //   print!("{}", (bitmask >> (i - 1)) & 1);
    // }
    // println!();

    if let Some(can_win) = computed[bitmask] {
      return can_win;
    }

    computed[bitmask] = Some(false);

    for i in 1..=choosable {
      let bit = (bitmask >> (i - 1)) & 1;

      if bit > 0 {
        continue;
      }

      let next_bitmask = bitmask | (1 << (i - 1));

      if current_sum + i >= desired {
        computed[next_bitmask] = Some(true);
        computed[bitmask] = Some(true);
        return true;
      }

      let can_opponent_win =
        Self::solve(choosable, desired, computed, next_bitmask, current_sum + i);

      if !can_opponent_win {
        computed[bitmask] = Some(true);
        break;
      }
    }

    computed[bitmask].unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::can_i_win(10, 11), false);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::can_i_win(10, 0), true);
  }

  #[test]
  fn test_example_3() {
    assert_eq!(Solution::can_i_win(10, 1), true);
  }
}
