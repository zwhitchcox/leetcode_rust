// use std::collections::HashMap;

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
