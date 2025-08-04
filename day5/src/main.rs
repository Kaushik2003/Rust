fn main() {
    let number = 3;
    if number > 5 {
        print!("condition is false");
    } else if number == 3 {
        print!("Same number")
    } else {
        println!("condition is true");
    }
    println!("Hello, world!");
    let truee = true;
    let _z = if truee { 10 } else { 6 };

    // standard way of writing a loop statement without any limit in Rust, but the below code is an infinite loop, so it's already telling us that any code below is unreachable.
   
    // loop {
    //     print!("Hi Im Kaushik");
    // };

    let mut count11 = 0;

    'counting_up: loop {
        println!("count = {count11}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count11 == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count11 += 1;
    }
    println!("End count = {count11}");


    // While Loop in 
    let mut number = 3;
    // The loop will continue as long as `number` is not 0.
    while number != 0 {
        println!("{}!", number);
        // Subtract 1 from number in each loop iteration.
        // This is crucial to prevent an infinite loop!
        number = number - 1;
    }



}
