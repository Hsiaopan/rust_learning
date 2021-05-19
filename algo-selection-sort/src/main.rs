pub fn selection_sort(vector: &mut Vec<usize>) -> &Vec<usize> {
    let len = vector.len();
    for i in 0..len {
        let mut tmp = i;
        for j in (i + 1)..len {
            if vector[tmp] > vector[j] {
                tmp = j;
            }
        }
        vector.swap(i, tmp);
    }
    vector
}

fn main() {
    let mut vec1 = vec![1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}", vec1);

    let vec2 = selection_sort(&mut vec1);
    println!("{:?}", vec2)
}
