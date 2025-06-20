pub struct Solution;

const BITS_SIZE: usize = i32::BITS as usize;

#[allow(dead_code)]
impl Solution {
  pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == 0 || divisor == 0 {
      return 0;
    } else if divisor == i32::MIN && dividend != i32::MIN {
      return 0;
    }

    let mut a = dividend;
    let mut b = divisor;

    let is_negative: bool = (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0);
    let mut addition = 0;

    if a == i32::MIN {
      addition = 1;
      if divisor < 0 {
        a -= b;
      } else {
        a += b;
      }
    }

    if a < 0 {
      a = -a;
    }

    if b == i32::MIN {
      b = i32::MAX; // Handle overflow case
    } else if b < 0 {
      b = -b;
    }

    let q_positive = Self::divide_positive(a, b);

    if is_negative {
      -q_positive - addition
    } else {
      if q_positive == i32::MAX {
        return i32::MAX;
      }
      q_positive + addition
    }
  }

  fn divide_positive(dividend: i32, divisor: i32) -> i32 {
    let mut quotient: i32 = 0;
    let mut remainder: i32 = 0;
    let bits_dividend = Self::extract_bits(dividend);

    for i in 1..BITS_SIZE {
      remainder = (remainder << 1) | bits_dividend[i] as i32;
      let mut quotient_bit: u8 = 0;
      if remainder >= divisor {
        quotient_bit = 1;
        remainder -= divisor;
      }
      quotient = (quotient << 1) | quotient_bit as i32;
    }

    quotient
  }

  fn extract_bits(value: i32) -> [u8; BITS_SIZE] {
    let mut bits = [0; BITS_SIZE];
    let mut d = value;
    for i in (0..BITS_SIZE).rev() {
      bits[i] = (d & 1) as u8;
      d >>= 1;
    }
    bits
  }

  //
  // fn twos_complement(bits: &[u8; BITS_SIZE]) -> [u8; BITS_SIZE] {
  //   let mut result = [0; BITS_SIZE];
  //   let mut carry = 1;
  //   for i in (0..BITS_SIZE).rev() {
  //     let inv = (!bits[i]) & 1; // Invert the bit
  //     let sum = inv + carry;
  //     result[i] = sum % 2;
  //     carry = sum / 2;
  //   }
  //   result
  // }

  // Solution that should not be accepted but still made it.
  // pub fn divide(dividend: i32, divisor: i32) -> i32 {
  //   dividend.checked_div(divisor).unwrap_or_else(|| {
  //     if (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0) {
  //       i32::MAX // Handle overflow case
  //     } else {
  //       i32::MIN // Handle overflow case
  //     }
  //   })
  // }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_divide() {
    assert_eq!(Solution::divide(-10, 2), -5);
    assert_eq!(Solution::divide(13, 2), 6);
    assert_eq!(Solution::divide(i32::MAX, -1), -2147483647);
    assert_eq!(Solution::divide(i32::MAX, 1), 2147483647);
    assert_eq!(Solution::divide(i32::MIN, -1), 2147483647);
    assert_eq!(Solution::divide(i32::MIN, 1), -2147483648);
    assert_eq!(Solution::divide(-1010369383, -2147483648), 0);
    assert_eq!(Solution::divide(-2147483648, -2147483648), 1);
    assert_eq!(Solution::divide(2147483647, -2147483648), 0);
  }

  #[test]
  fn test_divide_positive() {
    assert_eq!(Solution::divide_positive(10, 2), 5);
    assert_eq!(Solution::divide_positive(13, 2), 6);
    assert_eq!(Solution::divide_positive(i32::MAX, 2), 1073741823);
  }
}
