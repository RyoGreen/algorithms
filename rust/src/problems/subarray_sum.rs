pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            for j in i + 1..=nums.len() {
                let sub = &nums[i..j];
                if k == sub.iter().sum() {
                    result += 1
                }
            }
        }
        result
    }
}
