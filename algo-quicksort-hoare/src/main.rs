fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    let pivot = arr[lo as usize];
    let mut i = lo;
    let mut j = hi;

    loop {
        // find element >= pivot from leftmost element.
        while arr[i as usize] < pivot {
            i += 1;
        }

        // find element <= pivot from rightmost element.
        while arr[j as usize] > pivot {
            j -= 1;
        }

        if i >= j {
            return j;
        }

        arr.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
}

fn quicksort_helper(arr: &mut [i32], mut lo: isize, mut hi: isize) {
    while lo < hi {
        let pivot = partition(arr, lo, hi);
        if pivot - lo < hi - pivot {
            quicksort_helper(arr, lo, pivot - 1);
            lo = pivot + 1;
        } else {
            quicksort_helper(arr, pivot + 1, hi);
            hi = pivot - 1;
        }
    }
}

pub fn quicksort_hoare(arr: &mut [i32]) -> &mut [i32] {
    let hi = arr.len() as isize - 1;
    quicksort_helper(arr, 0, hi);
    arr
}

fn main() {
    let mut arr1 = [1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}", arr1);
    println!("{:?}", quicksort_hoare(&mut arr1))
}
