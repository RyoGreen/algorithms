pub fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return mid as i32;
        }
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}

pub fn find_first_occurrence(arr: &[i32], target: i32) -> isize {
    let mut left = 0;
    let mut right = arr.len() as isize - 1;
    let mut result = -1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid as usize] == target {
            result = mid;
            right = mid - 1;
        }
        if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

pub fn search_range(arr: &[i32], target: i32) -> Vec<i32> {
    let first = find_first_occurrence(arr, target);
    let last = find_last_occurrence(arr, target);

    vec![first as i32, last as i32]
}

fn find_last_occurrence(arr: &[i32], target: i32) -> isize {
    let mut left = 0;
    let mut right = arr.len() as isize - 1;
    let mut result = -1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid as usize] == target {
            result = mid;
            left = mid + 1;
        }
        if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}
