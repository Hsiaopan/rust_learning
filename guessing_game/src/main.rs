use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guess_try: u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess_try += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You win!, total tries: {}", guess_try);
                break;
            }
        }
        
    }
}
