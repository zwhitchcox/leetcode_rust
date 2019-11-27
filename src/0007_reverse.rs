// Given a 32-bit signed integer, reverse digits of an integer.
//
// Example 1:
//
// Input: 123
// Output: 321
// Example 2:
//
// Input: -123
// Output: -321
// Example 3:
//
// Input: 120
// Output: 21
// Note:
// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.

pub struct Solution;

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut r: i64 = 0;
    let mut x = x as i64;
    while x != 0 {
      r = r * 10 + (x % 10) as i64;
      x /= 10;
    }
    if r as i32 as i64 != r {
      return 0;
    }
    r as i32
  }
}

pub fn main() {
  println!("result:\n{}", Solution::reverse(-321),);
  println!("result:\n{}", Solution::reverse(1534236469),);
}

mod test {
  #[test]
  fn basics() {}
}
