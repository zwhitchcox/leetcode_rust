pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let halfway = total_len / 2;
        let mut nums1_i = 0;
        let mut nums2_i = 0;
        
        let mut next = || -> i32 {
            if let Some(nums1_val) = nums1.get(nums1_i) {
                if let Some(nums2_val) = nums2.get(nums2_i) {
                    if nums1_val < nums2_val {
                        nums1_i += 1;
                        *nums1_val
                    } else {
                        nums2_i += 1;
                        *nums2_val
                    }
                } else {
                    nums1_i += 1;
                    *nums1_val
                }
            } else {
                let result = nums2[nums2_i];
                nums2_i += 1;
                result
            }
        };

        
        if total_len % 2 == 0 {
            for _i in 0..(halfway - 1) {
                next();
            }
            let r1 = next();
            let r2 = next();
            (r1 + r2) as f64 / 2.0
        } else {
            for _i in 0..(halfway) {
                let r1 = next();
            }
            next() as f64
        }
    }
}

pub fn main() {
    let mut result = Solution::find_median_sorted_arrays(vec![1,3],vec![2]);
    println!("result: {}", result);
    result = Solution::find_median_sorted_arrays(vec![1,3],vec![2, 4]);
    println!("result: {}", result);

}


mod test {
    #[test]
    fn basics() {
    }
}

