const MAX: usize = 10007;

#[derive(Clone, Copy)]
struct Entry {
  k: i32,
  v: i32,
}

impl Default for Entry {
  fn default() -> Self {
    Entry { k: -1, v: -1 }
  }
}

struct MyHashMap {
  data: [Entry; MAX],
  deleted: [bool; MAX],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyHashMap {
  // Using a simple hash function h(k) = k mod m, where m is a prime number (in this case: MAX).
  // Using linear probing for collision resolution.
  fn new() -> Self {
    MyHashMap {
      data: [Entry::default(); MAX],
      deleted: [false; MAX],
    }
  }

  fn put(&mut self, key: i32, value: i32) {
    let ent = Entry { k: key, v: value };
    let i = self.hash(key);
    let mut found: Option<usize> = None;

    for d in 0..MAX {
      let idx = (i + d) % MAX;
      let is_empty = self.is_empty(idx);
      let is_deleted = self.is_deleted(idx);
      let is_same_key = self.is_same_key(idx, key);
      if is_empty || is_deleted || is_same_key {
        found = Some(idx);
        break;
      }
    }

    if let Some(idx) = found {
      self.data[idx] = ent;
      self.deleted[idx] = false;
      return;
    }
  }

  fn get(&self, key: i32) -> i32 {
    let i = self.hash(key);

    for d in 0..MAX {
      let idx = (i + d) % MAX;
      let is_empty = self.is_empty(idx);
      if is_empty {
        return -1;
      }
      let is_deleted = self.is_deleted(idx);
      let is_same_key = self.is_same_key(idx, key);
      if is_same_key && !is_deleted {
        return self.data[idx].v;
      }
    }

    -1
  }

  fn remove(&mut self, key: i32) {
    let i = self.hash(key);

    for d in 0..MAX {
      let idx = (i + d) % MAX;
      let is_empty = self.is_empty(idx);
      if is_empty {
        return;
      }
      let is_deleted = self.is_deleted(idx);
      let is_same_key = self.is_same_key(idx, key);
      if is_same_key && !is_deleted {
        self.deleted[idx] = true;
        return;
      }
    }
  }

  fn hash(&self, key: i32) -> usize {
    key as usize % MAX
  }

  fn is_empty(&self, idx: usize) -> bool {
    self.data[idx].k == -1
  }

  fn is_deleted(&self, idx: usize) -> bool {
    self.deleted[idx]
  }

  fn is_same_key(&self, idx: usize, key: i32) -> bool {
    self.data[idx].k == key
  }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_my_hash_map() {
    let mut obj = MyHashMap::new();
    obj.put(1, 2);
    assert_eq!(obj.get(1), 2);
    obj.remove(1);
    assert_eq!(obj.get(1), -1);
    obj.put(111111, 42);
    assert_eq!(obj.get(111111), 42);
  }
}
