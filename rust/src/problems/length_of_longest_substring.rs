use std::{cmp::max, collections::HashMap};

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut result: i32 = 0;
        let mut start_index: i32 = 0;

        for (index, c) in s.chars().enumerate() {
            if let Some(&prev_index) = map.get(&c) {
                if prev_index as i32 >= start_index {
                    start_index = prev_index as i32 + 1;
                }
            }
            map.insert(c, index);
            result = max(result, index as i32 - start_index + 1);
        }

        result
    }
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        1
    }
}
