use rust::problems::length_of_longest_substring::Solution;

#[test]
fn test_length_of_longest_substring() {
    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3); // "abc"

    let s = "abcdef".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 6); // "abcdef"

    let s = "bbbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1); // "b"

    let s = "".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 0); // ""

    let s = "a".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1); // "a"

    let s = "pwwkew".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3); // "wke"

    let s = "a".repeat(10_000);
    assert_eq!(Solution::length_of_longest_substring(s), 1); // "a"

    let s = "ab".repeat(5_000);
    assert_eq!(Solution::length_of_longest_substring(s), 2); // "ab"
}
