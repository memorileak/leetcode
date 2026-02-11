pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Ranging string S from i to j: S[i..=j] (i, j are 0-based);
  // Call L[i][j] the length of longest palindromic subsequence in S[i..=j]
  // L[i][i]   = 1 for every i from 0 to n-1
  // L[i][i+1] = 2 if S[i] == S[i+1]
  //           = 1 if S[i] != S[i+1]
  // L[i][j]   = 2 + L[i+1][j-1] if S[i] == S[j]
  //           = Max(L[i][j-1], L[i+1][j]) if S[i] != S[j]

  // Space optimization:
  // L[i][j] is computed based on:
  //   L[i+1][j-1] - diagonal (bottom left)
  //   L[i][j-1]   - left
  //   L[i+1][j]   - below
  // So we can optimize by using only 2 rows:
  // Above (A - smaller i) and Below (B -larger i)

  // For i from n-1 down to 0:
  //   For j from i to n-1:
  //     A[j]   = 1 if j == i
  //     A[j]   = 2 if j == i+1 && S[i] == S[j]
  //            = 1 if j == i+1 && S[i] != S[j]
  //     A[j]   = 2 + B[j-1] if j > i+1 && S[i] == S[j]
  //            = Max(A[j-1], B[j]) if j > i+1 S[i] != S[j]

  // Program carefully, we can use only 1 array for both A and B
  // In each iteration of i, when computing A[j]:
  //   Unmodified A[j] is B[j] (below)
  //   At that time, A[j-1] is overwritten, it becomes (left)
  //   So we need 1 extra variable to store the previous value of A[j-1] which is (diagonal)

  pub fn longest_palindrome_subseq(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();

    if n == 1 {
      return 1;
    }

    let mut a: Vec<u16> = vec![0; n];
    let mut diag: u16;

    for i in (0..n).rev() {
      a[i] = 1;
      if i < n - 1 {
        a[i + 1] = if s[i] == s[i + 1] { 2 } else { 1 };
      }
      // Reset the diag to L[i+1][j-1], but j == i+2, so it's L[i+1][i+1] (= 1)
      diag = 1;
      for j in (i + 2)..n {
        let next_diag = a[j];
        if s[i] == s[j] {
          a[j] = 2 + diag;
        } else {
          a[j] = a[j - 1].max(a[j]);
        }
        diag = next_diag;
      }
    }

    a[n - 1] as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_longest_palindrome_subseq() {
    // Example 1: One possible longest palindromic subsequence is "bbbb"
    assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);

    // Example 2: One possible longest palindromic subsequence is "bb"
    assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);

    // Example 3: Longest palindromic subsequence is "aaa"
    assert_eq!(Solution::longest_palindrome_subseq("aaa".to_string()), 3);
  }
}
