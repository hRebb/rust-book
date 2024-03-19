use std::io;

fn main() {
    println!("Guess the number");
    println!("Enter your solution :");

    let mut guess = String::new;

    io::stdin
        .read_lines(&mut guess)
        .expect("You failed");

    println!("You guessed : {guess}");
}