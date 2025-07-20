use rust::problems::zigzag_conversion::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: &'static str,
        num_rows: i32,
        expected: &'static str,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                input: "PAYPALISHIRING",
                num_rows: 3,
                expected: "PAHNAPLSIIGYIR",
            },
            TestCase {
                input: "PAYPALISHIRING",
                num_rows: 4,
                expected: "PINALSIGYAHRPI",
            },
        ]
    }

    #[test]
    fn test_convert() {
        for (i, case) in test_cases().iter().enumerate() {
            let result = Solution::convert(case.input.to_string(), case.num_rows);
            assert_eq!(
                result, case.expected,
                "convert failed at case {}: input='{}', num_rows={}",
                i, case.input, case.num_rows
            );
            let result = Solution::convert_v2(case.input.to_string(), case.num_rows);
            assert_eq!(
                result, case.expected,
                "convert_v2 failed at case {}: input='{}', num_rows={}",
                i, case.input, case.num_rows
            );
        }
    }
}
