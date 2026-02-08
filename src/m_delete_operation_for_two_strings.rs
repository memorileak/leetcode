pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn min_distance(word1: String, word2: String) -> i32 {
    (word1.len() + word2.len()) as i32 - 2 * Self::lcs(&word1, &word2)
  }

  // Find LCS length of word1 and word2
  // Let L[i][j] be the length of LCS between word1[1..=i] and word2[1..=j] (i, j are 1-based).
  // L[0][j] = 0;
  // L[i][0] = 0;
  // If word1[i] == word2[j]
  //   L[i][j] = L[i-1][j-1] + 1;
  // If word1[i] != word2[j]
  //   L[i][j] = Max(
  //     L[i-1][j],
  //     L[i][j-1]
  //   )
  //
  // Space optimization:
  //   L[i]: Current -> C
  //   L[i-1]: Prev -> P
  //   C[0] = 0
  //   If word1[i] == word2[j]
  //     C[j] = P[j-1] + 1
  //   If word1[i] != word2[j]
  //     C[j] = Max(P[j], C[j-1])
  fn lcs(word1: &str, word2: &str) -> i32 {
    let x = word1.as_bytes();
    let y = word2.as_bytes();
    let n = x.len();
    let m = y.len();

    let mut a: Vec<i32> = vec![0; m + 1];
    let mut b: Vec<i32> = vec![0; m + 1];
    let mut c = &mut a;
    let mut p = &mut b;

    for i in 1..=n {
      for j in 1..=m {
        if x[i - 1] == y[j - 1] {
          c[j] = p[j - 1] + 1;
        } else {
          c[j] = p[j].max(c[j - 1]);
        }
      }
      std::mem::swap(&mut c, &mut p);
    }

    p[m]
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_min_distance() {
    let word1 = "sea".to_string();
    let word2 = "eat".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 2);

    let word1 = "leetcode".to_string();
    let word2 = "etco".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 4);
  }
}
