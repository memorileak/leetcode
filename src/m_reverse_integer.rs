pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut pos_x: i32 = x.checked_abs().unwrap_or(0);
    if pos_x == 0 {
      return 0;
    }

    let mut rev_x: i32 = 0;
    let mut exp = pos_x.ilog10() as i32;

    while pos_x > 0 && exp >= 0 {
      let last_digit = pos_x % 10;
      if let Some(addition) = last_digit.checked_mul(10_i32.pow(exp as u32)) {
        if let Some(new_rev_x) = rev_x.checked_add(addition) {
          rev_x = new_rev_x;
        } else {
          return 0;
        }
      } else {
        return 0;
      }
      pos_x /= 10;
      exp -= 1;
    }

    if x < 0 {
      rev_x = -rev_x;
    }

    rev_x
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_reverse() {
    assert_eq!(Solution::reverse(99), 99);
    assert_eq!(Solution::reverse(100), 1);
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(1534236469), 0); // Overflow case
    assert_eq!(Solution::reverse(-2147483648), 0); // Overflow case
  }
}
