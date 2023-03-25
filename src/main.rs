use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Try to guess the number 1 - 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    println!("Please input your guess");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = guess.trim().parse().expect("Please type only numbers!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You got it!")
}


    println!("You guessed: {guess}");
    println!("Actual number: {secret_number}")


}
