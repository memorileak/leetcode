use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let l = begin_word.len(); // Max l is 10
    let word_list_len = word_list.len(); // Max word_list_len is 5000

    let begin = Self::word_u128(&begin_word);
    let end = Self::word_u128(&end_word);
    let mut words: Vec<u128> = vec![];

    let mut src: usize = word_list_len;
    let mut tgt: usize = word_list_len;

    for (i, w) in word_list.into_iter().enumerate() {
      let w_u128 = Self::word_u128(&w);
      words.push(w_u128);
      if end == w_u128 {
        tgt = i;
      } else if begin == w_u128 {
        src = i;
      }
    }

    if tgt == word_list_len {
      // No end_word found in word_list
      return 0;
    }

    if src == word_list_len {
      // No begin_word found in word_list
      words.push(begin);
    }

    let graph = Self::build_graph(&words, l);

    // BFS on the graph to find the shortest path from src to tgt
    let mut queue: VecDeque<(i32, usize)> = VecDeque::new();
    let mut seen: HashSet<usize> = HashSet::new();

    queue.push_back((1, src));
    seen.insert(src);

    while !queue.is_empty() {
      let (level, node) = queue.pop_front().unwrap();
      for &nei in graph[node].iter() {
        if !seen.contains(&nei) {
          if nei == tgt {
            return level + 1;
          }
          queue.push_back((level + 1, nei));
          seen.insert(nei);
        }
      }
    }

    0
  }

  pub fn word_bytes(word_str: &str) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    let str_bytes = word_str.as_bytes();
    bytes[..str_bytes.len()].copy_from_slice(str_bytes);
    bytes
  }

  pub fn word_u128(word_str: &str) -> u128 {
    u128::from_le_bytes(Self::word_bytes(word_str))
  }

  pub fn word_covr(word_u128: u128, covr_byte_i: u128) -> u128 {
    let k: u128 = !(0xFF << (8 * covr_byte_i));
    word_u128 & k
  }

  pub fn build_graph(words: &Vec<u128>, wordlen: usize) -> Vec<Vec<usize>> {
    let n = words.len(); // Max n is 5001
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut neimap: HashMap<u128, Vec<usize>> = HashMap::new();

    for i in 0..wordlen {
      let i_u128 = i as u128;
      for k in 0..n {
        // Cover words[k] at byte i, all words have same mapkey are neighbors (connected)
        let mapkey = Self::word_covr(words[k], i_u128);
        neimap.entry(mapkey).or_default().push(k);
      }
      for (_, neinodes) in neimap.iter() {
        let neilen = neinodes.len();
        for m in 0..neilen {
          for n in 0..neilen {
            if m != n {
              graph[neinodes[m]].push(neinodes[n]);
            }
          }
        }
      }
      neimap.clear();
    }

    graph
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_word_covr() {
    assert_eq!(Solution::word_covr(0x123456, 0), 0x123400);
    assert_eq!(Solution::word_covr(0x123456, 1), 0x120056);
    assert_eq!(Solution::word_covr(0x123456, 2), 0x003456);
  }

  #[test]
  fn test_build_graph() {
    let words = vec![
      Solution::word_u128("hit"),
      Solution::word_u128("hot"),
      Solution::word_u128("dot"),
      Solution::word_u128("dog"),
      Solution::word_u128("cog"),
    ];
    let mut graph = Solution::build_graph(&words, 3);
    for i in 0..graph.len() {
      graph[i].sort();
    }
    assert_eq!(graph[0], vec![1]);
    assert_eq!(graph[1], vec![0, 2]);
    assert_eq!(graph[2], vec![1, 3]);
    assert_eq!(graph[3], vec![2, 4]);
    assert_eq!(graph[4], vec![3]);
  }

  #[test]
  fn test_ladder_length_example1() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec![
      "hot".to_string(),
      "dot".to_string(),
      "dog".to_string(),
      "lot".to_string(),
      "log".to_string(),
      "cog".to_string(),
    ];
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
  }

  #[test]
  fn test_ladder_length_example2() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec![
      "hot".to_string(),
      "dot".to_string(),
      "dog".to_string(),
      "lot".to_string(),
      "log".to_string(),
    ];
    assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
  }
}
