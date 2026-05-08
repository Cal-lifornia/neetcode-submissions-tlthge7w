impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = nums.clone();
        insertion_sort(&mut arr);
        arr
    }
}

fn insertion_sort(arr: &mut [i32]) {
    (1..arr.len()).for_each(|idx| {
        let key = arr[idx];
        let mut j = idx as isize - 1;
        while j >= 0 && key < arr[j as usize] {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    });
}


