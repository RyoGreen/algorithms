pub struct Solution;

impl Solution {
    pub fn move_zeros(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        for i in 0..nums.len() - 1 {
            if nums[i] != 0 {
                break;
            }
            for j in i..nums.len() - 1 {
                (nums[j], nums[j + 1]) = (nums[j + 1], nums[j])
            }
        }
    }
}
