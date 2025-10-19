pub struct Solution;

impl Solution {
    pub fn move_zeros(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1
            }
        }
        for j in i..nums.len() {
            nums[j] = 0;
        }
    }
}
