// Control Flow: if else

fn main() {
    let x: i32 = 60;

    if x >= 18 {
        println!("You are an eligible to get a driving license");
    } else {
        println!("You are a not eligible to get a driving license");
    }

    let y: i32 = 55;
    if y >= 85 {
        println!("Distinction");
    } else if y >= 60 {
        println!("First Class");
    } else if y >= 50 {
        println!("Second Class");
    } else if y >= 35 {
        println!("Pass");
    } else {
        println!("Fail");
    }

    let number = if x > 50 { 5 } else { 1 };
    println!("The value of number is: {}", number);
}
