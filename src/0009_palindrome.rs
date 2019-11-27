// Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.

// Example 1:

// Input: 121
// Output: true
// Example 2:

// Input: -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
// Example 3:

// Input: 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
// Follow up:

// Coud you solve it without converting the integer to a string?

pub struct Solution;

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
      return false;
    }

    if x < 10 {
      return true;
    }

    if x % 10 == 0 {
      return false;
    }

    let mut y = x;
    let mut z = 0;

    while y != 0 {
      z = z * 10 + y % 10;
      y /= 10;
    }

    z == x
  }
}

pub fn main() {
  println!("result:\n{}\n{}", Solution::is_palindrome(121), true);
  println!("result:\n{}\n{}", Solution::is_palindrome(-42), false);
  println!("result:\n{}\n{}", Solution::is_palindrome(1001), true);
  println!("result:\n{}\n{}", Solution::is_palindrome(424), true);
  println!("result:\n{}\n{}", Solution::is_palindrome(425), false);
}

mod test {
  #[test]
  fn basics() {}
}
