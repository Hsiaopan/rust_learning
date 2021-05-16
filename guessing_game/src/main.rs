use std::io;

use rand::Rng;

fn main() {
    const MAX_TRIES: u32 = 5;
    println!("Guess the number!.You only have {} times", MAX_TRIES);

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut rest_tries: u32 = MAX_TRIES;

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

        rest_tries -= 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You win!, total tries: {}", MAX_TRIES - rest_tries);
                break;
            }
        }

        if 0.eq(&rest_tries) {
            println!("You lost.");
            break;
        };
        println!("You have {} times now", rest_tries);
    }
}
