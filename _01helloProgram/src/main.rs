fn main() {
    println!("Hello, Rust form Cargo!");

    let x = 5;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    test(THREE_HOURS_IN_SECONDS);
    test2();
    let y: i32 = test3(5);
    println!("The value of y is: {y}");

    counter();

    variable_and_data();

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn test(three_hours_in_seconds: u32) {
    println!("Three hours in seconds: {}", three_hours_in_seconds);
}

fn test2() {
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn test3(x: i32) -> i32 {
    let y: i32 = x * 5;
    y
}

fn counter() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        println!("The value of counter is: {counter}");
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn variable_and_data() {
    let s1: String = String::from("hello");
    let s2: String = s1;

    println!("{s2}, world!");

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");
    }

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
