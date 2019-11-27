pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut digit_count = 1 + (x as f32).log10() as u32;
        let mut i = 0;
        loop {
            if digit_count < 2 {
                return true;
            }
            let most_significance = digit_count + i - 1;
            let most_significant = (x / 10_i32.pow(most_significance)) % 10;
            let least_significance = i;
            let least_significant = (x / 10_i32.pow(least_significance)) % 10;
            i += 1;
            digit_count -= 2;
            if most_significant != least_significant {
                return false;
            }
        }
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
