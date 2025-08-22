use rust::problems::max_area_of_island::Solution;

#[test]
fn test_max_area_of_island() {
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::max_area_of_island(grid), 6);

    let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(Solution::max_area_of_island(grid), 0);

    let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(Solution::max_area_of_island(grid), 1);

    let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
    assert_eq!(Solution::max_area_of_island(grid), 9);

    let grid = vec![
        vec![1, 0, 0, 1],
        vec![1, 0, 0, 1],
        vec![0, 0, 0, 0],
        vec![1, 1, 0, 0],
    ];
    assert_eq!(Solution::max_area_of_island(grid), 2);
}
