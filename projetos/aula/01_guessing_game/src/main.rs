//GUESSING GAME
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("**GUESS THE NUMBER!**");
    
    //generating random number
    let secret_number = rand::thread_rng()
                                .gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    //keyboard input
    println!("Please, input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //parse to integer (shadowing)
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    println!("You guessed: {}", guess);

    //comparing guess with secret_number
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("To small!"),
        Ordering::Greater => println!("To big!"),
        Ordering::Equal => println!("You win!"),
    } //match-cmp
}
