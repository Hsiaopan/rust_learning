pub fn binary_insertion_sort(vectors: &mut Vec<usize>) -> &Vec<usize> {
    for i in 1..vectors.len() {
        let val = vectors[i];
        let mut j = i;
        let pos = match vectors[..i].binary_search(&val) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        while j > pos {
            vectors.swap(j - 1, j);
            j -= 1;
        }
    }
    vectors
}

fn main() {
    let mut vec1 = vec![1, 5, 7, 3, 9, 11, 2, 3, 6, 12, 8];
    println!("{:?}", vec1);

    let vec2 = binary_insertion_sort(&mut vec1);
    println!("{:?}", vec2);
}
