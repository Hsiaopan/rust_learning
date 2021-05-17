fn main() {
    let my_string = String::from("Hello world");
    let word = first_word(&my_string);
    println!("the first word is: {}", word);

    let mys_string_literal = "hello world";
    let word = first_word(&mys_string_literal[..]);
    println!("the first word is: {}", word);

    let word = first_word(mys_string_literal);
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
