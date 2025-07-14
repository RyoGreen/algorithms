use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut m = HashMap::new();
        for c in s.chars() {
            if let Some(x) = m.get(&c) {
                m.insert(c, x + 1);
            } else {
                m.insert(c, 1);
            }
        }
        for (i, c) in s.chars().enumerate() {
            if m.get(&c) == Some(&1) {
                return i as i32;
            }
        }
        -1
    }
}
