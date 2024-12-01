use rust::problems::sort::Solution;

#[test]
fn test_quick_sort() {
    let nums = vec![15, 11, 7, 2];
    let sorted = Solution::quick_sort(nums.clone());
    assert_eq!(sorted, vec![2, 7, 11, 15]);
}

#[test]
fn test_bubble_sort() {
    let nums = vec![15, 11, 7, 2];
    let sorted = Solution::bobble_sort(nums.clone());
    assert_eq!(sorted, vec![2, 7, 11, 15]);
}
