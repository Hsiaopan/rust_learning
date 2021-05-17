fn main() {
    let my_string = String::from("Hello world");
    let word = first_word(&my_string); // first_word 中传入 `String` 的 slice
    println!("the first word is: {}", word);

    let mys_string_literal = "hello world";
    let word = first_word(&mys_string_literal[..]); // first_word 中传入字符串字面值的 slice
    println!("the first word is: {}", word);

    let word = first_word(mys_string_literal); // 不使用 slice 语法
    println!("The first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
