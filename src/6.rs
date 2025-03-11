use std::io;

fn main() {
    let mut guess = String::new();

    println!("Please enter your guess.");

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please enter a number");

    if guess == 42 {
        println!("You're right!");
    } else {
        println!("Sorry, you're wrong.");
    }
}
