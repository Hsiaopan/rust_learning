use std::usize;

pub fn insertion_sort(arr: &mut Vec<usize>) {
    for i in 1..arr.len() {
        let (mut p, v) = (i, arr[i]);
        while p > 0 && arr[p - 1] > arr[p] {
            arr[p] = arr[p-1];
            p -= 1;
        }
        arr[p] = v;
    }
}

fn main() {
    let mut arr = vec![1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}",insertion_sort(&mut arr));
}
