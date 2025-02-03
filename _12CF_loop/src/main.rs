// Loops : for, while, loop

fn main() {
    // loop without condition breaks the loop
    // loop {
    //     println!("Hello, world!");
    // }

    let mut number = 5;
    let result = loop {
        number += 1;
        if number == 10 {
            break number * 2;
        }
    };
    println!("The result is {}", result);

    // while loop

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let array = [10, 20, 30, 40, 50];
    let fruits = ["apple", "banana", "cherry", "date", "elderberry"];
    for element in array {
        println!("The value is: {}", element);
    }

    for furit in fruits {
        println!("The fruit is: {}", furit);
    }
}
