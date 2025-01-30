use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    // let secret_number = rand::thread_rng().gen_range(1..=100);

    let secret_number = rand::rng().random_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u16 = guess.trim().parse().expect("Please type a number!");

    println!("The secret number is: {}", secret_number);

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    let secret_number = secret_number + 12;
    {
        let secret_number = secret_number * 2;
        println!("The multiplied secret number is: {}", secret_number);
    }
    println!("The new secret number is: {}", secret_number);

}
