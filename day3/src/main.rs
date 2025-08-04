//* */ Common Programming Concepts

// fn main() {
//     let age: i32 = 24;
//     const PI: i32 = 108000;
//     println!("{}", age * PI);
// }

//* Shadowing */
// you can declare a new variable with the same variable name

fn main() {
    let apples = 10;
    println!("apples is {}", apples);
    let apples = 20;
    println!("apples is {}", apples);
    apples = 20;
    println!("apples is {}", apples);
}
 