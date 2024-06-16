pub fn binary_search_last(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return mid as i32;
        }
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    -1
}
