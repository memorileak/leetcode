use std::{
  collections::HashMap,
  hash::{BuildHasherDefault, Hasher},
};

// Build my own u128 hasher, basically no hashing at all
#[derive(Default)]
pub struct U128Hasher(u64);

impl Hasher for U128Hasher {
  // Boilerplate: We don't expect this method to be called for u128 keys
  fn write(&mut self, _: &[u8]) {
    unreachable!("Only for u128");
  }

  // Fold 128 bits into 64 bits
  fn write_u128(&mut self, i: u128) {
    let low = i as u64;
    let high = (i >> 64) as u64;
    self.0 = low ^ high;
  }

  fn finish(&self) -> u64 {
    self.0
  }
}

pub type FastU128Map<V> = HashMap<u128, V, BuildHasherDefault<U128Hasher>>;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  const N: usize = 16;

  pub fn longest_str_chain(words: Vec<String>) -> i32 {
    let mut ordered_words: Vec<Vec<&str>> = vec![vec![]; Self::N];
    for word in words.iter() {
      ordered_words[word.len() - 1].push(word);
    }

    let mut c: FastU128Map<i32> = FastU128Map::default();
    let mut max: i32 = 0;

    for words_vec in ordered_words.into_iter() {
      for word in words_vec.iter() {
        let keys = Self::gen_keys(word);
        let mut chain_len: i32 = 1;

        for k in keys.iter().skip(1) {
          let len: i32 = *(c.get(k).unwrap_or(&0));
          chain_len = chain_len.max(len + 1);
        }

        c.insert(keys[0], chain_len);
        max = max.max(chain_len);
      }
    }

    max
  }

  fn gen_keys(word: &str) -> Vec<u128> {
    let mut result: Vec<u128> = vec![];
    let word_bytes = word.as_bytes();
    let word_key = Self::str_bytes_to_u128(word_bytes);
    result.push(word_key);

    let n = word.len();
    if n > 1 {
      for i in 0..n {
        let high_mask = if i < (Self::N - 1) {
          u128::MAX << 8 * (i + 1)
        } else {
          0
        };
        let high: u128 = (word_key & high_mask) >> 8;
        let low_mask = (!high_mask) >> 8;
        let low: u128 = word_key & low_mask;
        let pred_key = high | low;
        result.push(pred_key);
      }
    }

    result
  }

  fn str_bytes_to_u128(s_bytes: &[u8]) -> u128 {
    let l = s_bytes.len().min(16);
    let mut bytes: [u8; 16] = [0; 16];
    bytes[..l].copy_from_slice(&s_bytes[..l]);
    u128::from_ne_bytes(bytes)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::longest_str_chain(vec![
        "a".to_string(),
        "b".to_string(),
        "ba".to_string(),
        "bca".to_string(),
        "bda".to_string(),
        "bdca".to_string(),
      ]),
      4
    );
    assert_eq!(
      Solution::longest_str_chain(vec![
        "xbc".to_string(),
        "pcxbcf".to_string(),
        "xb".to_string(),
        "cxbc".to_string(),
        "pcxbc".to_string(),
      ]),
      5
    );
    assert_eq!(
      Solution::longest_str_chain(vec!["abcd".to_string(), "dbqca".to_string(),]),
      1
    );
    assert_eq!(
      Solution::longest_str_chain(
        vec![
          "bqtmbnugq",
          "bjqtmbnuwsgq",
          "m",
          "btmnugq",
          "czlhk",
          "ihkgszexnh",
          "wiszechhcxldcrow",
          "kgszn",
          "lhk",
          "zlzfgmjurcntwvn",
          "jjvodlrsmdmie",
          "tm",
          "ihqkgpszexnh",
          "wnwpxg",
          "zmun",
          "hkgszenh",
          "zmucnwn",
          "kgzn",
          "yjmk",
          "wiszechcxldcrow",
          "fnnh",
          "yjm",
          "jjvodlrmdmie",
          "bqtmnugq",
          "hkgszen",
          "wehcxdrow",
          "nhed",
          "zmucn",
          "wisehcxdrow",
          "fnnlh",
          "wehcxdro",
          "zlzgmjurcntwvn",
          "wnwg",
          "jjvodlmde",
          "wisechcxldcrow",
          "wiehcxdrow",
          "nhxyeedlcqxw",
          "wehcxr",
          "zzgmjurcnwvn",
          "btmgq",
          "nwdhslknbwpxg",
          "ihqkgszexnh",
          "jjvodlrsmdmhie",
          "jjvodlmd",
          "wdhslknbwpxg",
          "nhedxw",
          "wisehcxldcrow",
          "btmugq",
          "mfnnlfh",
          "bqtmbnuwgq",
          "nhedcxw",
          "bqtmbnuwsgq",
          "nhe",
          "zzgmjurcntwvn",
          "jjvodlrmdie",
          "whslknwpxg",
          "wdhslknwpxg",
          "wsnwpxg",
          "jjvodlm",
          "hkgszexnh",
          "zzgmjucnwvn",
          "nhxyeedlcxw",
          "nhxedcxw",
          "zzmucnwn",
          "hkgszn",
          "zmucnw",
          "whsnwpxg",
          "czlmhk",
          "whsknwpxg",
          "wehcxro",
          "wnwpg",
          "nhxeedcxw",
          "nwdhslknbjwpxg",
          "nhedw",
          "tmg",
          "zlhk",
          "zlzfgmjurcntwvnr",
          "jjvodlmdie",
          "zzmjucnwvn",
          "mfnnlh",
          "zzmjucnwn",
          "wisehcxldrow",
          "tmgq",
          "nhxyeedcxw",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
      ),
      13
    );
  }
}
