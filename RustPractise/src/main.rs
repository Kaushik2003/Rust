//* Giving and retrieving Ownership
// fn main() {
//     let s = String::from("Kaushik");

//     let (s, len) = calculate_length(s);// we shadowed the first variable
//     println!("The length of {s} is {len} ");
// }

// fn calculate_length(s: String) -> (String, usize)// here we took the ownnership of the S string and and then returned the ownership
// {
//     let result = s.len();
//     (s, result)
// }

//* Passing References and Borrowing

// ~ Immutable Reference
// fn main() {
//     let s = String::from("Kaushik");
//     let len = calculate_length(&s);

//     println!("The length of {s} is {len}");
// }

// fn calculate_length(s: &String) ->usize{
//     let len = s.len();
//     len
// }

// ~ Mutable Reference
// fn main() {
//     let mut s = String::from("Kaushik");
//     let len = calculate_length(&mut s);

//     println!("The length of {s} is {len}");
// }

// fn calculate_length(s: &mut String) ->usize{
//     s.push_str(" Samadder");
//     let len = s.len();
//     len
// }

fn main(){
let is_even=true;

if(is_even){
    
}
}