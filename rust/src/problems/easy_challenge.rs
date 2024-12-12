use std::collections::{HashMap, HashSet};

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
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut result: i32 = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                result += prices[i] - prices[i - 1];
            }
        }
        result
    }
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (k as usize) % nums.len();
        nums.reverse();
        nums[0..k].reverse();
        nums[k..].reverse();
    }
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in nums {
            if map.entry(n).or_insert(()) == &() {
                return true;
            }
        }
        false
    }

    pub fn contains_duplicate_v2(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for n in nums {
            if !set.insert(n) {
                return true;
            }
        }
        false
    }
}
