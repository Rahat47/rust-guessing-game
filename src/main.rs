use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // let user decide lives
    println!("How many lives do you want to play with?");
    let mut lives_input = String::new();

    io::stdin().read_line(&mut lives_input).expect("Failed to read line");

    let mut lives: u32 = match lives_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("You have {lives} lives. Good luck!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        attempts += 1;
        println!("\nLives remaining: {lives}");
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                lives -= 1; 
                if lives > 0 {
                    println!("Too small! Try again.");
                }
            },
            Ordering::Greater => {
                lives -= 1; 
                if lives > 0 {
                    println!("Too big! Try again.");
                }
            },
            Ordering::Equal => {
                println!("You win! The secret number was {secret_number}.");
                println!("You took {attempts} attempts!");
                break;
            }
        }

        // Check if player ran out of lives
        if lives == 0 {
            println!("Game Over! You ran out of lives.");
            println!("The secret number was: {secret_number}");
            break;
        }
    }
}
