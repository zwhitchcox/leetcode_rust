pub struct Solution;

fn find_ltr(answer: &mut Vec<Vec<i32>>, nums: &Vec<i32>, i: usize, j: usize) {
    if j - i < 2 {
        return;
    }
    let mut moving = j - 1;
    let first_sum = nums[i] + nums[j];
    loop {
        if i == moving {
            break;
        }
        let sum = first_sum + nums[moving];
        if sum < 0 {
            break;
        }
        if sum == 0 {
            answer.push(vec![nums[i], nums[moving], nums[j] ]);
            break;
        }
        moving -= 1;
    }
}

fn find_rtl(answer: &mut Vec<Vec<i32>>, nums: &Vec<i32>, i: usize, j: usize) {
    if j - i < 2 {
        return;
    }
    let mut moving = i + 1;
    let first_sum = nums[i] + nums[j];
    loop {
        if j == moving {
            break;
        }
        let sum = first_sum + nums[moving];
        if sum > 0 {
            break;
        }
        if sum == 0 {
            answer.push(vec![nums[i], nums[moving], nums[j] ]);
            break;
        }
        moving += 1;
    }
}

impl Solution {
    pub fn three_sum(im_nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        if im_nums.len() < 3 {
            return answer;
        }
        if im_nums.len() == 3 {
            if (im_nums[0] + im_nums[1] + im_nums[2]) == 0 {
                answer.push(vec![im_nums[0], im_nums[1], im_nums[2]]);
            }
            return answer;
        }
        let mut nums = im_nums.clone();
        nums.sort();
        let mut i = 0;
        let mut j = nums.len() - 1;
        while (i + 1) < j {
            let first_sum = nums[i] + nums[j];
            if first_sum < 0 {
                find_ltr(&mut answer, &nums, i, j);
                let cur_i = nums[i];
                i += 1;
                while nums[i] == cur_i && i + 1 < j {
                    i += 1;
                }
            }
            if first_sum > 0 {
                find_rtl(&mut answer, &nums, i, j);
                let cur_j = nums[j];
                j -= 1;
                while nums[j] == cur_j  && i + 1 < j{
                    j -= 1;
                }
            }
            if first_sum == 0 {
                let first_i = i;
                find_ltr(&mut answer, &nums, i, j);
                let cur_i = nums[i];
                i += 1;
                while nums[i] == cur_i && i + 1 < j {
                    i += 1;
                }
                if (i + 1) > j {
                    break;
                }
                find_ltr(&mut answer, &nums, i, j);
                let end = nums[j];
                j -= 1;
                while nums[j] == end  && i + 1 < j {
                    j -= 1;
                }
                if (i + 1) > j {
                    break;
                }
                find_rtl(&mut answer, &nums, first_i, j);
            }
        }
        answer
    }
}

pub fn main() {
    println!(
        "result:\n{:?}\n{:?}",
        Solution::three_sum(vec![-2, -1, 0, 1, 2, 3]),
        vec![
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-1, 0, 1],
        ],
    );
    println!(
        "result:\n{:?}\n{:?}",
        Solution::three_sum(vec![-2, 0, 1, 1, 2]),
        vec![
            vec![-2, 0, 2],
            vec![-2, 1, 1],
        ],
    );
    println!(
        "result:\n{:?}\n{:?}",
        Solution::three_sum(vec![-2, 0, 0, 2, 2]),
        vec![
            vec![-2, 0, 2],
        ],
    );
    println!(
        "result:\n{:?}\n{:?}",
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![
            vec![-1, 0, 1],
            vec![-1, -1, 2]
        ],
    );
    println!(
        "result:\n{:?}\n{:?}",
        Solution::three_sum(vec![1, 1, 1]),
        vec![""],
    );
    println!(
        "result:\n{:?}\n{:?}",
        Solution::three_sum(vec![0, 0, 0]),
        vec![
            vec![0, 0, 0]
        ],
    );
    println!(
        "result:\n{:?}\n{:?}",
        Solution::three_sum(vec![0, 0, 0, 0]),
        vec![
            vec![0, 0, 0]
        ],
    );
    println!(
        "result:\n{:?}\n{:?}",
        Solution::three_sum(vec![]),
        vec![""],
    );
}

mod test {
    #[test]
    fn basics() {}
}
