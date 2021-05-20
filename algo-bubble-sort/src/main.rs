pub fn bubble_sort(arr: &mut [usize]) -> &mut [usize] {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true
            }
        }
    }
    arr
}

pub fn bubble_sort_optimized(arr: &mut [usize]) -> &mut [usize] {
    let mut new_len: usize;
    let mut len = arr.len();
    loop {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 {
            break;
        }
        len = new_len;
    }
    arr
}

fn main() {
    let mut arr1 = [1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}", arr1);
    println!("{:?}", bubble_sort(&mut arr1));

    println!("optimized: \n{:?}", bubble_sort_optimized(&mut arr1));
}
