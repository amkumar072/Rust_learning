// shadowing: re-declaring a variable with the same name

fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x initially: {}", x);

    {
        let x = x * 2;
        println!("The value of x in inner scope: {}", x);
    }
    let x = 10;
    println!("The value of x finally: {}", x);

    let x = "string";
    println!("The value of x as a string: {}", x);

    let x: String = String::from("string");
    println!("The value of x as a string: {}", x);
}
