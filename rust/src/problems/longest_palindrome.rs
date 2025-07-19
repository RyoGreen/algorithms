pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let mut length = 0;

        for i in 0..len {
            let mut left = i as i32;
            let mut right = i as i32;
            while left >= 0
                && (right as usize) < len
                && chars[left as usize] == chars[right as usize]
            {
                if (right - left + 1) > length {
                    length = right - left + 1;
                }
                left -= 1;
                right += 1;
            }

            let mut left = i as i32;
            let mut right = i as i32 + 1;
            while left >= 0
                && (right as usize) < len
                && chars[left as usize] == chars[right as usize]
            {
                if (right - left + 1) > length {
                    length = right - left + 1;
                }
                left -= 1;
                right += 1;
            }
        }

        length
    }
}
