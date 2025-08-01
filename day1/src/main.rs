// This is a guessing game in rust

use std::io;

fn main() {
    println!("Guess the number ! :)");   
    let mut _guess= String::new();//ine has created a mutable variable that is currently bound to a new, empty instance of a String
    let _apples =5;
    let mut _bananas=5;
    io::stdin().read_line(&mut _guess).expect("Failed to read line");

}

