use rust::problems::sort::Solution;

#[test]
fn test_quick_sort() {
    let nums = vec![15, 11, 7, 2];
    let sorted = Solution::quick_sort(nums.clone());
    assert_eq!(sorted, vec![2, 7, 11, 15]);
}

#[test]
fn test_quick_sort_already_sortred() {
    let nums = vec![1, 2, 3, 4];
    let sorted = Solution::quick_sort(nums.clone());
    assert_eq!(sorted, vec![1, 2, 3, 4]);
}
#[test]
fn test_bubble_sort() {
    let nums = vec![15, 11, 7, 2];
    let sorted = Solution::bubble_sort(nums.clone());
    assert_eq!(sorted, vec![2, 7, 11, 15]);
}

#[test]
fn test_quick_sort_v2() {
    let nums = vec![4, 3, 5, 9, 1];
    let sorted = Solution::quick_sort_v2(nums.clone());
    assert_eq!(sorted, vec![1, 3, 4, 5, 9]);
}
