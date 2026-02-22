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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3); // "abc"

        let s = "abcdef".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 6); // "abcdef"

        let s = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1); // "b"

        let s = "".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 0); // ""

        let s = "a".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1); // "a"

        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3); // "wke"

        let s = "a".repeat(10_000);
        assert_eq!(Solution::length_of_longest_substring(s), 1); // "a"

        let s = "ab".repeat(5_000);
        assert_eq!(Solution::length_of_longest_substring(s), 2); // "ab"
    }
}
