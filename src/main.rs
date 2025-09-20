use std::io;
use rand::Rng;

fn main(){
    let mut guess: String =  String::new();
    let number  = rand::thread_rng().gen_range(1..10);

    loop 
    {    
        println!("guess the number!");
            
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        println!("You guessed: {}", guess);

        if ( guess < 1 || guess > 10) {
            println!("The number must be between 1 and 10");
            continue;
        } 
        else if (guess == number) {
            println!("You win!");
            break;
        } 
        else if (guess > number) {
            println!("number is smaller");
            continue;
        } 
        else {
            println!("number is larger");
            continue;
        }
    }


}