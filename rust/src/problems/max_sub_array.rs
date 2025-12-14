pub struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() < 1 {
            return 0;
        }
        let mut current_sum = nums[0];
        let mut max_sum = nums[0];
        for n in 1..nums.len() {
            current_sum = current_sum.max(0) + nums[n];
            max_sum = max_sum.max(current_sum);
        }
        max_sum
    }
}
