pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut result = 1;
        for n in 1..nums.len() {
            if nums[n - 1] != nums[n] {
                nums[result] = nums[n];
                result += 1;
            }
        }
        result as i32
    }
}
