pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t_iter = t.chars();
        for c in s.chars() {
            if !t_iter.any(|x| x == c) {
                return false;
            }
        }
        true
    }
}
