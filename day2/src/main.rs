// std is the standard input/output library in Rust.By default. rust has a set of items that are included the program by default,Which is called the prelude.

// use std::io;

use rand;
use std::cmp::Ordering;
use std::io;

//* */ A simple program to demonstrate the guessing game in Rust.
fn main() {
    println!("Guess a Number !");

    let secret_number: i32 = rand::random_range(1..=10);
    print!("The secret numner is {}", secret_number);

    loop {
        println!("Please input Your number : ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Please enter a correct number {}", err);
                continue;
            }
        };
        //we used something called shadowing here...we shadowed the previous same named variable called "guess"

        println!("You guessed {}", guess);

        //match is very exhaustive . all code must be written

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed it right");
                break;
            }
            Ordering::Greater => println!("You guessed it higher"),
            Ordering::Less => println!("You guessed it lower"),
        };
    }
}
