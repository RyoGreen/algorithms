pub struct Solution;
pub use std::collections::{HashMap, HashSet};

impl Solution {
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
