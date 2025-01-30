fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    rectangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangle() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", &rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
