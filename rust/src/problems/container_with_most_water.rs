use std::cmp;
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area: i32 = 0;
        for (i, v) in height.iter().enumerate() {
            for n in i..height.len() {
                let tmp = (n - i) as i32 * cmp::min(height[n], *v);
                if tmp > area {
                    area = tmp;
                }
            }
        }
        area
    }
}
