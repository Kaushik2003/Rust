// The main function is the entry point of every Rust program.
fn main() {
    // --- 1. Basic `if-else` Conditional ---
    // `if-else` allows you to run code based on a condition.
    // The first block whose condition is true gets executed.

    let number = 3;

    if number > 5 {
        // This block runs if `number` is greater than 5.
        print!("condition is false"); // Note: `print!` does not add a new line.
    } else if number == 3 {
        // This block runs if the first condition is false AND `number` is equal to 3.
        print!("Same number");
    } else {
        // This block runs if none of the above conditions are true.
        println!("condition is true"); // `println!` adds a new line at the end.
    }

    // --- 2. Using `if` in a `let` Statement ---
    // In Rust, `if` is an expression, which means it can return a value.
    // This is a concise way to assign a value based on a condition.

    println!("\nHello, world!"); // Adding a newline for cleaner output.
    let truee = true;

    // The value of `_z` will be 10 if `truee` is true, otherwise it will be 6.
    // The underscore `_` before `z` tells the compiler we know this variable isn't used.
    let _z = if truee { 10 } else { 6 };
    // println!("The value of _z is: {}", _z); // Would print 10

    // --- 3. A Basic (Infinite) `loop` ---
    // A `loop` keyword creates a loop that runs forever until you explicitly tell it to stop
    // using the `break` keyword. The code below is commented out because it would run forever.

    // loop {
    //     print!("This would print forever! ");
    // };

    // --- 4. Nested Loops and Loop Labels ---
    // You can label loops to specify which loop you want to `break` or `continue` from.
    // This is useful for controlling nested loops.

    let mut count11 = 0;

    // We label the outer loop `'counting_up`.
    'counting_up: loop {
        println!("Outer loop count = {}", count11);
        let mut remaining = 10;

        // This is the inner loop.
        loop {
            println!("  Inner loop remaining = {}", remaining);

            if remaining == 9 {
                // `break` without a label exits the *inner* loop only.
                break;
            }
            if count11 == 2 {
                // `break 'counting_up'` exits the *outer* loop (the one with the label).
                break 'counting_up;
            }
            remaining -= 1;
        }

        count11 += 1;
    }
    println!("End count = {}\n", count11); // The final value of count11 will be 2.

    // --- 5. `while` Loop for a Countdown ---
    // A `while` loop runs as long as its condition remains true.

    let mut number_while = 3; // Renamed to avoid shadowing previous `number`

    while number_while != 0 {
        println!("{}!", number_while);
        number_while -= 1; // Same as `number_while = number_while - 1;`
    }
    println!(""); // Newline for spacing

    // --- 6. `while` Loop to Iterate Through an Array (Manual way) ---
    // This is a more "manual" way to loop through a collection. It works, but it's
    // prone to errors (e.g., if the index goes out of bounds).

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // The condition relies on knowing the array size.
        println!("The value at index {} is: {}", index, a[index]);
        index += 1;
    }
    println!("");

    // --- 7. `for` Loop to Iterate Through an Array (The Rust way) ---
    // A `for` loop is the preferred, safer, and more readable way to iterate over a collection.
    // It handles all the indexing logic for you, preventing common bugs.

    let a = [10, 20, 30, 40, 50]; // Shadowing the previous `a` is fine here.

    for element in a {
        // `for` takes each element from `a` one by one.
        println!("The value is: {}", element);
    }
    println!("");

    // --- 8. `for` Loop with a Range ---
    // You can use a `for` loop with a number range.

    // `1..=10` is an inclusive range, so it includes numbers from 1 to 10.
    println!("Counting up from 1 to 10:");
    for x in 1..=10 {
        println!("x is {}", x);
    }
    println!("");

    // You can use `.rev()` on a range to reverse the order.
    println!("Counting down from 10 to 1:");
    for x in (1..=10).rev() {
        println!("x is {}", x);
    }
}
