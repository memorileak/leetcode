pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

// Definition for singly-linked list.
#[allow(dead_code)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

#[allow(dead_code)]
impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut carry: i32 = 0;
    let mut n1 = l1.as_ref();
    let mut n2 = l2.as_ref();
    let mut head: Option<Box<ListNode>> = None;
    let mut tail = &mut head;

    while n1.is_some() || n2.is_some() || carry > 0 {
      let mut s: i32 = 0;

      if let Some(v1) = n1 {
        s += v1.val;
        n1 = v1.next.as_ref();
      }

      if let Some(v2) = n2 {
        s += v2.val;
        n2 = v2.next.as_ref();
      }

      s += carry;
      carry = s / 10;

      let new_node = Box::new(ListNode::new(s % 10));

      if tail.is_none() {
        head = Some(new_node);
        tail = &mut head;
      } else if let Some(t) = tail {
        t.next = Some(new_node);
        tail = &mut t.next;
      }
    }

    head
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_two_numbers() {
    // Input: l1 = [2,4,3], l2 = [5,6,4]
    // Output: [7,0,8]
    // Explanation: 342 + 465 = 807.  }
    //
    let l1 = Some(Box::new(ListNode {
      val: 2,
      next: Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode { val: 3, next: None })),
      })),
    }));

    let l2 = Some(Box::new(ListNode {
      val: 5,
      next: Some(Box::new(ListNode {
        val: 6,
        next: Some(Box::new(ListNode { val: 4, next: None })),
      })),
    }));

    let result = Solution::add_two_numbers(l1, l2);

    println!("{:?}", result);

    assert_eq!(result.unwrap().val, 7);
  }

  #[test]
  fn test_add_two_numbers_ninenine() {
    let l1 = Some(Box::new(ListNode {
      val: 9,
      next: Some(Box::new(ListNode {
        val: 9,
        next: Some(Box::new(ListNode { val: 9, next: None })),
      })),
    }));

    let l2 = Some(Box::new(ListNode { val: 1, next: None }));

    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", result);
    assert_eq!(result.unwrap().val, 0);
  }
}
