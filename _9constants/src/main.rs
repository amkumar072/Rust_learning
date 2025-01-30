// constants are always immutable

fn main() {
    println!("Hello, world!");

    const FIVE: i32 = 5;
    const SIX: &str = "SIX";

    println!("The value of FIVE is: {}", FIVE);
    println!("The keyword of SIX is: {}", SIX);
}
