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
    pub fn max_area_v2(height: Vec<i32>) -> i32 {
        let (mut area, mut left, mut right) = (0, 0, height.len() - 1);
        while right > left {
            let tmp = (right - left) as i32 * (cmp::min(height[left], height[right]));
            area = cmp::max(tmp, area);
            if height[right] > height[left] {
                left += 1
            } else {
                right -= 1
            }
        }
        area
    }
}
