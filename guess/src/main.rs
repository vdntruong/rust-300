use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number");

    // the kind of range expresstion here take the form start..=end
    let secret_number = thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess) // get input from user
            .expect("Failed to read lie"); // handle potential failure

        // parse to a number, Rust allows us to shadow the previous value of guess with a new one
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // we also can handle the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // over the loop again
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}
