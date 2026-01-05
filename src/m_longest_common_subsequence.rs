use std::mem::swap;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Longest Common Subsequence is one of the classic Dynamic Programming problems
  // (along with Rod Cutting, Matrix Chain Multiplication, 0/1 Knapsack, etc.).
  // Let c[i][j] be the length of the longest common subsequence of
  // the first i characters of text1 and the first j characters of text2:
  //   c[i][j] = 0 if i == 0 or j == 0
  //   c[i][j] = c[i-1][j-1] + 1 if both i, j > 0 and text1[i] == text2[j]
  //   c[i][j] = max(c[i-1][j], c[i][j-1]) if both i, j > 0 and text1[i] != text2[j]
  pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let s1 = text1.as_bytes();
    let s2 = text2.as_bytes();

    let m = s1.len();
    let n = s2.len();

    let mut x: [i32; 1001] = [0; 1001];
    let mut y: [i32; 1001] = [0; 1001];

    let mut c = &mut x;
    let mut p = &mut y;

    for i in 1..=m {
      for j in 1..=n {
        if s1[i - 1] == s2[j - 1] {
          c[j] = p[j - 1] + 1;
        } else {
          c[j] = p[j].max(c[j - 1]);
        }
      }

      swap(&mut c, &mut p);
    }

    if m % 2 == 0 { y[n] } else { x[n] }
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_longest_common_subsequence() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    let result = Solution::longest_common_subsequence(text1, text2);
    assert_eq!(result, 3);

    let text1 = "abc".to_string();
    let text2 = "abc".to_string();
    let result = Solution::longest_common_subsequence(text1, text2);
    assert_eq!(result, 3);

    let text1 = "abc".to_string();
    let text2 = "def".to_string();
    let result = Solution::longest_common_subsequence(text1, text2);
    assert_eq!(result, 0);

    let dna1 = "ACCGGTCGAGTGCGCGGAAGCCGGCCGAA".to_string();
    let dna2 = "GTCGTTCGGAATGCCGTTGCTCTGTAAA".to_string();
    // Result should be "GTCGTCGGAAGCCGGCCGAA", length 20
    let result = Solution::longest_common_subsequence(dna1, dna2);
    assert_eq!(result, 20);
  }
}
