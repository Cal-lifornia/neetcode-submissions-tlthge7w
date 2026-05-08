impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = nums.clone();
        quicksort(&mut arr);
        arr
    }
}

// Using the quicksort example from the following site:
// https://www.educative.io/blog/sorting-algorithms-for-developers#Solving-the-temperature-problem-using-the-quicksort
// &
// https://researchdatapod.com/quick-sort-rust/
fn quicksort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let pi = partition(arr);
        quicksort(&mut arr[0..pi]);
        quicksort(&mut arr[pi + 1..]);
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let high = arr.len() - 1;
    let pivot = arr[high];
    let mut i = 0;
    (0..high).for_each(|j| {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    });

    arr.swap(i, high);
    i
}
