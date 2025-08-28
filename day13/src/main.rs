
use std::env;

fn main() {
    // unsafe{env::set_var("RUST_BACKTRACE", "1");}
    unsafe{env::set_var("RUST_BACKTRACE", "full");}
    panic!("crash and burn");

    let arr=[1,2,3,4];
    let er=arr[10];
    println!("the {er}");
    
}