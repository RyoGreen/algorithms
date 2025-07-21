pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let is_nagative = x < 0;
        let mut x_abs = x.abs();
        let mut result: i32 = 0;
        while x_abs != 0 {
            let digit = x_abs % 10;
            x_abs /= 10;

            result = match result.checked_mul(10).and_then(|r| r.checked_add(digit)) {
                Some(r) => r,
                None => return 0,
            };
        }

        if is_nagative == true {
            result = -result
        }
        result
    }
}
