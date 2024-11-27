use rust::problems::two_sum::Solution;

#[test]
fn test_two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 1]);
}
