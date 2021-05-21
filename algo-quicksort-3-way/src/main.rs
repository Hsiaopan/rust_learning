fn partition(arr: &mut [i32], lo: isize, hi: isize) -> (isize, isize) {
    let pivot = arr[hi as usize];
    let mut i = lo;
    let mut j = lo;
    let mut k = hi;

    while j <= k {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
            j += 1;
        } else if arr[j as usize] > pivot {
            arr.swap(k as usize, j as usize);
            k -= 1;
        } else {
            // no swap when identicial
            j += 1;
        }
    }

    // Return smaller and larger pointer to avoid iterate duplicate element.
    (i, k)
}

pub fn quicksort_3way_optimized(arr: &mut [i32]) -> &mut [i32] {
    let hi = arr.len() as isize - 1;
    quicksort_helper_optimized(arr, 0, hi);
    arr
}

fn quicksort_helper_optimized(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {
        let (i, k) = partition(arr, lo, hi);
        if i - lo < hi - k {
            quicksort_helper_optimized(arr, lo, i - 1);
            quicksort_helper_optimized(arr, k + 1, hi);
        } else {
            quicksort_helper_optimized(arr, k + 1, hi);
            quicksort_helper_optimized(arr, lo, i - 1);
        }
    }
}

fn main() {
    let mut arr1 = [1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}", arr1);
    println!("optimized: \n{:?}", quicksort_3way_optimized(&mut arr1));
}
