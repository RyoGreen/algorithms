use rust::problems::move_zeros::Solution;

#[test]
fn test_move_zeros() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeros(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);
}
