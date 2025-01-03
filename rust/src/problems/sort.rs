pub struct Solution;

impl Solution {
    pub fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
        fn quicksort(arr: &mut [i32]) {
            if arr.len() <= 1 {
                return;
            }
            let pivot_index = partition(arr);
            quicksort(&mut arr[0..pivot_index]);
            quicksort(&mut arr[pivot_index + 1..]);
        }

        fn partition(arr: &mut [i32]) -> usize {
            let pivot = arr[arr.len() - 1];
            let mut i = 0;
            for j in 0..arr.len() - 1 {
                if arr[j] < pivot {
                    arr.swap(i, j);
                    i += 1;
                }
            }
            arr.swap(i, arr.len() - 1);
            i
        }

        quicksort(&mut arr);
        arr
    }
    pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let length = arr.len();
        for i in 0..length {
            for j in 0..(length - i - 1) {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
        arr
    }
}
