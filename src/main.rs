// use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ans = 0;
        let should_negate = x < 0;
        let mut x = x.abs();
        while x > 0 {
            ans *= 10;
            ans += x % 10;
            x /= 10;
        }
        if should_negate {
            ans *= -1;
        }
        ans
    }
}

pub fn main() {
    println!("result:\n{}", Solution::reverse(1534236469),);
}

mod test {
    #[test]
    fn basics() {}
}
