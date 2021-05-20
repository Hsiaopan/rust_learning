pub const MARCIN_GAPS: [usize; 8] = [701, 301, 132, 57, 23, 10, 4, 1];

pub fn shellsort(arr: &mut [usize]) -> &mut [usize] {
    let len = arr.len();

    // 迭代gap sequence
    for gap in MARCIN_GAPS.iter() {
        // 利用trait iter()产生迭代器，迭代gap sequence
        let mut i = *gap; // dereferemce

        // 迭代整个序列
        while i < len {
            let mut j = i;

            // 每个元素的插入排序动作
            while j > *gap && arr[j - gap] > arr[j] {
                // 插入
                arr.swap(j - *gap, j);
                j -= *gap;
            }
            i += 1
        }
    }
    arr
}

fn main() {
    let mut arr1 = [1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}", arr1);
    println!("{:?}", shellsort(&mut arr1));
}
