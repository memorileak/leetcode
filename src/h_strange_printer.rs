pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Notice that aaabbbbaa takes the exact same number of turns as aba.
  // The printer prints sequences of identical characters in one go.
  // Pre-process the string to remove consecutive duplicates: aaabbbbaaccc -> abac
  //
  // Let m[i][j] be the minimum turns needed to print the substring s[i..=j]
  // Look at the character s[j], it will have 2 options to get printed:
  //
  // 1. s[j] gets printed separately:
  //   We need to print the substring s[i..j-1],
  //   Then take 1 turn to print s[j] separately.
  //   Minimum number of turns: m[i][j-1] + 1
  //
  // 2. s[j] gets printed as a part of an earlier stroke:
  //   We look inside the range [i..=j-1] to see if there is any character s[k] that s[k] == s[j]
  //   If so, it implies we could have printed a single stroke from k all the way to j initially.
  //   We print the segment s[i...k]. (The stroke for s[k] implicitly extends to s[j]).
  //   We print the segment s[k+1...j-1] on top of that background stroke.
  //   Minimum number of turns: m[i][k] + m[k+1][j-1]
  //   We take the minimum over all possible k where s[k] == s[j]
  //
  // Therefore, the recurrence relation becomes:
  //   m[i][j] = Min(
  //     m[i][j-1] + 1,
  //     m[i][k] + m[k+1][j-1] for all k in [i..=j-1] where s[k] == s[j],
  //   )
  // Base case: m[i][i] = 1 for all i

  const N: usize = 128;

  pub fn strange_printer(s: String) -> i32 {
    let mut s_bytes: Vec<u8> = Vec::new();
    let mut c: u8 = 0;

    for &b in s.as_bytes() {
      if b == c {
        continue;
      }
      s_bytes.push(b);
      c = b;
    }

    let s_len = s_bytes.len();
    let mut m: [[u16; Self::N]; Self::N] = [[0; Self::N]; Self::N];

    for i in 0..s_len {
      m[i][i] = 1;
    }

    for l in 1..s_len {
      for i in 0..s_len - l {
        let j = i + l;
        m[i][j] = m[i][j - 1] + 1;
        for k in i..=j - 1 {
          if s_bytes[k] == s_bytes[j] {
            m[i][j] = m[i][j].min(m[i][k] + m[k + 1][j - 1]);
          }
        }
      }
    }

    m[0][s_len - 1] as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_strange_printer() {
    let s = "aaabbb".to_string();
    assert_eq!(Solution::strange_printer(s), 2);

    let s = "ababca".to_string();
    assert_eq!(Solution::strange_printer(s), 4);
  }
}
