use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input some guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read user input");

    println!("Did you guess: {guess}")
}
