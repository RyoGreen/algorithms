use rust::problems::max_sub_array::Solution;

#[test]
fn test_max_sub_array() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );

    assert_eq!(Solution::max_sub_array(vec![5]), 5);

    assert_eq!(Solution::max_sub_array(vec![-5]), -5);

    assert_eq!(Solution::max_sub_array(vec![-8, -3, -6, -2, -5, -4]), -2);

    assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4]), 10);

    assert_eq!(Solution::max_sub_array(vec![4, -10, 3, 5]), 8);

    assert_eq!(Solution::max_sub_array(vec![]), 0);
}
