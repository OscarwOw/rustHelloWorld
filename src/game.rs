use rand::Rng;
use std::io;

pub fn play() {
    let number: i8 = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Guess the number!");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number (1-10).");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        if guess < 1 || guess > 10 {
            println!("The number must be between 1 and 10");
            continue;
        } else if guess == number {
            println!("You win!");
            break;
        } else if guess > number {
            println!("number is smaller");
            continue;
        } else {
            println!("number is larger");
            continue;
        }
    }
}
