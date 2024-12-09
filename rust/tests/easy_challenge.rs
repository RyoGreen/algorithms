use rust::problems::easy_challenge::Solution;

#[test]
fn test_remove_duplicates() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
}
