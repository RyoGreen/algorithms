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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_zeros() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeros(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0, 0, 1];
        Solution::move_zeros(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);
    }
}
