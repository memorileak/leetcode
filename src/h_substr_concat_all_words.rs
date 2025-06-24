use std::collections::HashMap;
pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    if s.is_empty() || words.is_empty() {
      return vec![];
    }

    let mut words_simplified: Vec<String> = words.clone();

    // If all words are the same, we can treat it as a single word of all words
    if words.iter().all(|word| word == &words[0]) {
      let single_word = words.into_iter().collect::<String>();
      words_simplified = vec![single_word];
    }

    let mut appearances: Vec<i32> = vec![];
    let mut word_freq_map: HashMap<&str, i32> = HashMap::new();
    let total_words: usize = words_simplified.len();
    let word_length: usize = words_simplified.first().map_or(0, |w| w.len());
    let phrase_length: usize = total_words * word_length;

    if s.len() < phrase_length {
      return vec![]; // Not enough characters to form all words
    }

    for word in words_simplified.iter() {
      let count = word_freq_map.entry(word.as_str()).or_insert(0);
      *count += 1;
    }

    for i in 0..s.len() - phrase_length + 1 {
      let slice = &s[i..i + phrase_length];
      let mut wcount_map: HashMap<&str, i32> = HashMap::new();
      let mut matched_words: usize = 0;
      let mut quit_checking_current_slice = false;

      for j in (0..slice.len()).step_by(word_length) {
        let word = &slice[j..j + word_length];
        let count = wcount_map.entry(word).or_insert(0);
        *count += 1;
        if *count > *word_freq_map.get(word).unwrap_or(&0) {
          quit_checking_current_slice = true;
          break; // More occurrences than allowed, break early
        }
        matched_words += 1;
        if matched_words == total_words {
          appearances.push(i as i32);
          quit_checking_current_slice = true;
          break;
        }
      }

      if quit_checking_current_slice {
        continue; // Skip this slice if we already broke out
      }
    }

    appearances
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solution() {
    // Example 1
    assert_eq!(
      Solution::find_substring(
        "barfoothefoobarman".to_string(),
        vec!["foo".to_string(), "bar".to_string()]
      ),
      vec![0, 9]
    );

    // Example 2
    assert_eq!(
      Solution::find_substring(
        "wordgoodgoodgoodbestword".to_string(),
        vec![
          "word".to_string(),
          "good".to_string(),
          "best".to_string(),
          "word".to_string()
        ]
      ),
      vec![]
    );

    // Example 3
    assert_eq!(
      Solution::find_substring(
        "barfoofoobarthefoobarman".to_string(),
        vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
      ),
      vec![6, 9, 12]
    );

    // Additional test: Empty words
    assert_eq!(
      Solution::find_substring("hello".to_string(), vec![]),
      vec![]
    );

    // Additional test: Empty string
    assert_eq!(
      Solution::find_substring("".to_string(), vec!["word".to_string()]),
      vec![]
    );

    // Additional test: Single word
    assert_eq!(
      Solution::find_substring("hello".to_string(), vec!["hello".to_string()]),
      vec![0]
    );
  }
}
