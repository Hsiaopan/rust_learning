#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn another() {
        assert_ne!(4, add_two(2));
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
