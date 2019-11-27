pub struct Solution;

// 0ms, 2.6MB, 100%, 100%

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
      return s;
    }
    let num_rows = num_rows as usize;
    let mut rows = vec![String::new(); num_rows.min(s.len())];
    let mut row: i32 = 0;
    let mut increment = 1;
    for c in s.chars() {
      rows[row as usize].push(c);
      row += increment;
      if num_rows - 1 == row as usize || row == 0 {
        increment *= -1;
      }
    }

    let mut res = String::new();
    for row in &rows {
      res.push_str(row);
    }

    res
  }
}

pub fn main() {
  println!(
    "result:\n{}\n{}",
    Solution::convert("PAYPALISHIRING".to_string(), 3),
    "PAHNAPLSIIGYIR",
  );
  println!(
    "result:\n{}\n{}",
    Solution::convert("PAYPALISHIRING".to_string(), 4),
    "PINALSIGYAHRPI",
  );
  println!(
    "result:\n{}\n{}",
    Solution::convert("AB".to_string(), 1),
    "AB",
  );
}

mod test {
  #[test]
  fn basics() {}
}
