fn main() {
    hello_world();

    let _x = {
        let height: u32 = 100;
        let width: u32 = 50;
        height + width
    };
    println!("The value of _X is: {}", _x);

    let y = add(5, 10);
    println!("The value of y is: {}", y);
}

fn hello_world() {
    println!("Hello, world!");
    tell_height(99);
}

fn tell_height(height: u32) {
    println!("The height is: {}", height);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
