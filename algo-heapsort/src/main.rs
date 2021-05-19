pub fn heapsort(arr: &mut [i32]) -> &mut [i32] {
    // -- Heapify part --
    // This procedure would build a valid max-heap.
    // (or min-heap for sorting descendantly)
    let end = arr.len();
    for start in (0..end / 2).rev() {
        sift_down(arr, start, end - 1);
    }

    // -- Sorting part --
    // Iteratively sift down unsorted part (the heap).
    for end in (1..arr.len()).rev() {
        arr.swap(end, 0);
        sift_down(arr, 0, end - 1);
    }
    arr
}

fn sift_down(arr: &mut [i32], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }

        if child + 1 <= end && arr[child] < arr[child + 1] {
            child += 1;
        }

        if arr[root] < arr[child] {
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
fn main() {
    let mut vec1 = [1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}", vec1);
    println!("{:?}", heapsort(&mut vec1));
}
