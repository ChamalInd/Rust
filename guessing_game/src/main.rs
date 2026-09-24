use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Guessing Game!");
    print!("Please enter you guess: ");
    let _ = io::stdout().flush(); // helps to print the above text immediately
         
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line.");
    
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    println!("\nYou guessed: {guess}");
    println!("Secret number: {secret_number}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Number is too low!"),
        Ordering::Equal => println!("You guessed correct"),
        Ordering::Greater => println!("Number is too big!")
    }
}
