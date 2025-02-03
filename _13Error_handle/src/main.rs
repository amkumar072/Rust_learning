// Error handle
// 2 ways to handle error: Resutl<T, E> and Option<T>

// Sample Option
// enum Option<T> {
//     Some(T), // Some value
//     None,    // None value
// }

// Sample Result
// enum Result<T, E> {
//     Ok(T),  // Ok value
//     Err(E), // Err value
// }

fn main() {
    let result = divide(10.0, 0.0);
    match result {
        Some(value) => println!("The result is {}", value),
        None => println!("Cannot divide by zero"),
    }

    match divide_result(10.0, 5.0) {
        Ok(value) => println!("The result is {}", value),
        Err(message) => println!("{}", message),
    }
}

fn divide(num: f64, den: f64) -> Option<f64> {
    if den == 0.0 {
        None
    } else {
        Some(num / den)
    }
}

fn divide_result(num: f64, den: f64) -> Result<f64, String> {
    if den == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(num / den)
    }
}
