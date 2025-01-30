// Ownership in Rust is a unique feature that makes Rust stand out from other programming languages.
// Ownership is a set of rules that the compiler checks at compile time.

fn main() {
    let s1: String = String::from("hello");
    let s2: usize = calculate_length(&s1);
    println!("The length of '{s1}' is {s2}.");

    // borrowing
    borrow();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Borrowing
fn borrow() {
    let _x: i32 = 5;
    let _y: &i32 = &_x; // borrowing or referencing

    print!("The value of \n _x is {}\n _y is: {}\n", _x, _y);

    borrow_mut();
}

fn borrow_mut() {
    let mut x: i32 = 5;
    let y: &mut i32 = &mut x;
    *y += 5;
    println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y); // error: cannot borrow `y` as immutable because it is also borrowed as mutable
}
