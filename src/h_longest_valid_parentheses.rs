pub struct Solution;

#[allow(dead_code)]
impl Solution {
  pub fn longest_valid_parentheses(s: String) -> i32 {
    if s.len() == 0 {
      return 0;
    }

    let mut state: Vec<i8> = vec![0; s.len() as usize];
    let mut open_stack: Vec<usize> = vec![];
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in chars.into_iter().enumerate() {
      if c == '(' {
        open_stack.push(i);
        state[i] = 1;
      } else if c == ')' {
        if let Some(o) = open_stack.pop() {
          state[o] = 0;
          state[i] = 0;
        } else {
          state[i] = -1;
        }
      }
    }

    let mut current_vp: i32 = 0;
    let mut longest_vp: i32 = 0;
    for s in state.into_iter() {
      if s == 0 {
        current_vp += 1;
        longest_vp = longest_vp.max(current_vp);
      } else {
        current_vp = 0;
      }
    }

    longest_vp
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test_example_1() {
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
  }

  #[test]
  fn test_example_2() {
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
  }

  #[test]
  fn test_example_3() {
    assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
  }


  #[test]
  fn test_example_4() {
    assert_eq!(Solution::longest_valid_parentheses("(((((".to_string()), 0);
    assert_eq!(Solution::longest_valid_parentheses(")))))".to_string()), 0);
    assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses("(()()())((()()()()()".to_string()), 10);
  }
}
