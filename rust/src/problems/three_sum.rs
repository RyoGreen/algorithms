use std::cmp::Ordering;

pub struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        for i in 0..sorted_nums.len() {
            if sorted_nums[i] > 0 {
                break;
            }
            if i > 0 && sorted_nums[i] == sorted_nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = sorted_nums.len() - 1;

            while left < right {
                let sum = sorted_nums[i] + sorted_nums[left] + sorted_nums[right];
                match sum.cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![sorted_nums[i], sorted_nums[left], sorted_nums[right]]);
                        while left < right && sorted_nums[left] == sorted_nums[left + 1] {
                            left += 1;
                        }
                        while left < right && sorted_nums[right] == sorted_nums[right - 1] {
                            right -= 1;
                        }
                        left += 1;
                        right -= 1;
                    }
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                }
            }
        }
        result
    }
}
