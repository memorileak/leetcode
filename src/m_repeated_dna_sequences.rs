use std::collections::HashSet;

const BASE: u32 = 7;
const MOD_ALPHA: u32 = 1_000_000_007;
const MOD_BETA: u32 = 1_000_000_009;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Solve this problem using Rabin-Karp algorithm (Rolling Hash) & Double Hashing.
  // For each substring of length 10, compute its 2 hash values and store it in 2 different hash sets.
  // If a substring's hash values are already in both 2 hash sets, it means we have seen this substring before.
  // Otherwise, add the hash values to the respective hash sets.
  pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let length = s.len();
    if length < 10 {
      return vec![];
    }

    let mut result: HashSet<String> = HashSet::new();
    let mut seen_alpha: HashSet<u32> = HashSet::new();
    let mut seen_beta: HashSet<u32> = HashSet::new();

    let l: usize = 10;
    let first_slice = &s[0..l];

    let first_hash_alpha = Self::hash(MOD_ALPHA, first_slice);
    let mut last_hash_alpha = first_hash_alpha;
    seen_alpha.insert(first_hash_alpha);

    let first_hash_beta = Self::hash(MOD_BETA, first_slice);
    let mut last_hash_beta = first_hash_beta;
    seen_beta.insert(first_hash_beta);

    let s_chars: Vec<char> = s.chars().collect();
    for i in 1..=length - l {
      let left_char = s_chars[i - 1];
      let right_char = s_chars[i + l - 1];

      last_hash_alpha = Self::hash_rolling(MOD_ALPHA, last_hash_alpha, left_char, right_char, l);
      last_hash_beta = Self::hash_rolling(MOD_BETA, last_hash_beta, left_char, right_char, l);

      if seen_alpha.contains(&last_hash_alpha) && seen_beta.contains(&last_hash_beta) {
        let substring = &s[i..i + l];
        result.insert(substring.to_string());
        continue;
      }

      seen_alpha.insert(last_hash_alpha);
      seen_beta.insert(last_hash_beta);
    }

    result.into_iter().collect()
  }

  pub fn value(c: char) -> u32 {
    match c {
      'A' => 1,
      'C' => 2,
      'G' => 3,
      'T' => 4,
      _ => 0,
    }
  }

  pub fn hash(mod_val: u32, s: &str) -> u32 {
    let mut hash_value: u32 = 0;
    for c in s.chars() {
      hash_value = (hash_value * BASE + Self::value(c)) % mod_val;
    }
    hash_value
  }

  pub fn hash_rolling(
    mod_val: u32,
    prev_hash: u32,
    left_char: char,
    right_char: char,
    length: usize,
  ) -> u32 {
    let left_value = (Self::value(left_char) * BASE.pow(length as u32 - 1)) % mod_val;
    let mut new_hash = (prev_hash + mod_val - left_value) % mod_val; // Remove left char
    new_hash = (new_hash * BASE + Self::value(right_char)) % mod_val; // Shift left and add right char
    new_hash
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hash() {
    let s = "ACGT";
    let hash = Solution::hash(MOD_ALPHA, s);
    assert_eq!(hash, 466);
  }

  #[test]
  fn test_hash_rolling() {
    let s = "AAAAACCCCC";
    let new_s = "AAAACCCCCA";
    let hash = Solution::hash(MOD_ALPHA, s);
    let new_hash = Solution::hash_rolling(MOD_ALPHA, hash, 'A', 'A', 10);
    let expected_hash = Solution::hash(MOD_ALPHA, new_s);
    assert_eq!(new_hash, expected_hash);
  }

  #[test]
  fn test_find_repeated_dna_sequences() {
    let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
    let result = Solution::find_repeated_dna_sequences(s);
    let mut expected = vec!["AAAAACCCCC".to_string(), "CCCCCAAAAA".to_string()];
    expected.sort();
    let mut result_sorted = result.clone();
    result_sorted.sort();
    assert_eq!(result_sorted, expected);
  }
}
