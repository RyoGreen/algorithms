pub struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut rows = vec![String::new(); num_rows as usize];
        let mut i = 0;
        let mut d = 0;
        for c in s.chars() {
            rows[i as usize].push(c);
            if i == 0 {
                d = 1;
            } else if i == num_rows - 1 {
                d = -1;
            }
            i += d
        }

        rows.concat()
    }
    pub fn convert_v2(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let mut result = "".to_string();
        for (i, c) in s.char_indices() {
            if i == 0 {
                result.push(c);
            }
        }
        result
    }
}
