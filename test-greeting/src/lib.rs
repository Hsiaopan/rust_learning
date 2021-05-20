#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn another_greeting_contains_name() {
        let result = another_greeting("Bob");
        assert!(
            result.contains("Bob"),
            "Greeting did no contain name, value was '{}'",
            result
        );
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn another_greeting(name: &str) -> String {
    String::from("Hello!")
}
