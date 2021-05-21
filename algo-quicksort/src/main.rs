/// Recursion helper
fn quicksort_helper(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {
        let pivot = partition(arr, lo, hi);
        quicksort_helper(arr, lo, pivot - 1);
        quicksort_helper(arr, pivot + 1, hi);
    }
}

fn quicksort_helper_optimized(arr: &mut [i32], lo: isize, hi: isize) {
    if lo <= hi {
        let pivot = partition(arr, lo, hi);
        if pivot - lo < hi - pivot {
            quicksort_helper_optimized(arr, lo, pivot - 1);
            quicksort_helper_optimized(arr, pivot + 1, hi);
        } else {
            quicksort_helper_optimized(arr, pivot + 1, hi);
            quicksort_helper_optimized(arr, lo, pivot - 1);
        }
    }
}

// TCO
fn quicksort_helper_manual_tco(arr: &mut [i32], mut lo: isize, mut hi: isize) {
    while lo < hi {
        let pivot = partition(arr, lo, hi);
        if pivot - lo < hi - pivot {
            quicksort_helper_manual_tco(arr, lo, pivot - 1);
            lo = pivot + 1;
        } else {
            quicksort_helper_manual_tco(arr, pivot + 1, hi);
            hi = pivot - 1;
        }
    }
}

pub fn quicksort_lomuto(arr: &mut [i32]) -> &mut [i32] {
    let hi = arr.len() as isize - 1;
    quicksort_helper(arr, 0, hi);
    arr
}

pub fn quicksort_lomuto_optimized(arr: &mut [i32]) -> &mut [i32] {
    let hi = arr.len() as isize - 1;
    quicksort_helper_optimized(arr, 0, hi);
    arr
}

// TCO
pub fn quicksort_lomuto_manual_tco(arr: &mut [i32]) -> &mut [i32] {
    let hi = arr.len() as isize - 1;
    quicksort_helper_manual_tco(arr, 0, hi);
    arr
}

fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    // -- Deteimin the pivot --
    // In Lomuto partition scheme,
    // the latest element is always chosen as the pivot.
    let pivot = arr[hi as usize];
    let mut i = lo;

    // -- Swap elements --
    for j in lo..hi {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }

    // Swap pivot to the middle of two piles.
    arr.swap(i as usize, hi as usize);

    // return the final index of the pivot
    i
}

fn main() {
    let mut arr1 = [1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}", arr1);
    println!("{:?}", quicksort_lomuto(&mut arr1));

    println!("optimized: \n{:?}", quicksort_lomuto_optimized(&mut arr1));

    println!(
        "TCO optimized: \n{:?}",
        quicksort_lomuto_manual_tco(&mut arr1)
    );
}
