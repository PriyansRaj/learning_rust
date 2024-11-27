use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to the 'Guess the number' game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    loop {
        println!("Enter your guess (1-100): ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Enter a valid number.");
                continue;
            }
        };
        attempts += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win under {} attemps", attempts);
                break;
            }
            Ordering::Greater => println!("T00 big"),
        }
    }
}
