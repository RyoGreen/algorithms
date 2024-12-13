use rust::problems::three_sum::Solution;

#[test]
fn test_max_profit() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    assert_eq!(Solution::three_sum(nums), expected);

    let nums = vec![1, 2, 3];
    let expected: Vec<Vec<i32>> = Vec::new();
    assert_eq!(Solution::three_sum(nums), expected);

    let nums: Vec<i32> = Vec::new();
    let expected: Vec<Vec<i32>> = Vec::new();
    assert_eq!(Solution::three_sum(nums), expected);

    let nums = vec![-1, 0, 1, 2, -1, -4, 1, -1, -1, 0];
    let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    assert_eq!(Solution::three_sum(nums), expected);

    let nums = vec![-1, 0, 1, 2, -1, -4];
    let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    assert_eq!(Solution::three_sum(nums), expected);
}
