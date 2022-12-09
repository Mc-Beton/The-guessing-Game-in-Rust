use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the most common made game ever!");
    println!("The Guess my number Game!");
    println!("Let's generate a number to guess.. READY!");

    let number_to_guess = rand::thread_rng().gen_range(1..=100);
    let mut counter = 0;

    loop {
        println!("Please type in your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a valid number, please try again.");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
    
        println!("You guessed: {guess}");
        counter += 1;

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("Bingo! ({counter} atempts)");
                break;
            }
        }
    }
}