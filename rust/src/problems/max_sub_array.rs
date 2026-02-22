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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sub_array() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );

        assert_eq!(Solution::max_sub_array(vec![5]), 5);

        assert_eq!(Solution::max_sub_array(vec![-5]), -5);

        assert_eq!(Solution::max_sub_array(vec![-8, -3, -6, -2, -5, -4]), -2);

        assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4]), 10);

        assert_eq!(Solution::max_sub_array(vec![4, -10, 3, 5]), 8);

        assert_eq!(Solution::max_sub_array(vec![]), 0);
    }
}
