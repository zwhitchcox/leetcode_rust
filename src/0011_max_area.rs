use  std::cmp;
pub struct Solution;


impl Solution {
    pub fn max_area(lines: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = lines.len()  - 1;
        let mut max = 0;
        while j > i {
            let height = lines[i].min(lines[j]);
            let width = j - i;
            max = max.max(height * (width as i32));
            if lines[i] > lines[j] {
                j -= 1;
            } else {
                i += 1;
            }
        }
        
        max
    }
}

pub fn main() {
    println!(
        "result:\n{}\n{}",
        Solution::max_area(vec![1,8,6,2,5,4,8,3,7]),
        49
    );
    println!(
        "result:\n{}\n{}",
        Solution::max_area(vec![1,1]),
        1
    );
}

mod test {
    #[test]
    fn basics() {}
}
