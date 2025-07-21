pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let is_nagative = x < 0;
        let x_abs = x.abs();
        let reverse: String = x_abs.to_string().chars().rev().collect();
        let mut result: i32 = reverse.parse::<i32>().unwrap();

        if is_nagative == true {
            result = -result
        }
        result
    }
}
