// use std::collections::HashMap;

pub struct Solution;

// definitely room for improvement

fn find_palindrome(bytes: &[u8], len: isize, j: isize, k: isize) -> (usize, usize) {
  let (mut j, mut k) = (j, k);
  while j >= 0 && k < len && bytes[j as usize] == bytes[k as usize] {
    j -= 1;
    k += 1;
  }

  ((j + 1) as usize, (k - j - 1) as usize)
}

impl Solution {
  pub fn longest_palindrome(sv: String) -> String {
    let len = sv.len() as isize;
    if len < 2 {
      return sv;
    }
    let bytes: Vec<u8> = sv.bytes().collect();
    let iter_even = (0..len - 1).map(|i| find_palindrome(&bytes, len, i, i));
    let iter_odd = (0..len - 1).map(|i| find_palindrome(&bytes, len, i, i + 1));

    if let Some((start, max_sz)) = iter_even.chain(iter_odd).max_by_key(|&(_, sz)| sz) {
      sv[start..start + max_sz].into()
    } else {
      sv[0..1].into()
    }
  }
}

pub fn main() {
  // let mut result = Solution::longest_palindrome(String::from("badad"));
  // println!("result: {}", result);
  // result = Solution::longest_palindrome(String::from("ssaasdbc"));
  // println!("result: {}", result);
  let result = Solution::longest_palindrome(String::from("tattarrattat"));
  println!("result: {}", result);
}

mod test {
  #[test]
  fn basics() {}
}
