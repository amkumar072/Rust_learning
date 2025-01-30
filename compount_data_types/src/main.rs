// Compound Data Types
// Array, tuple, slice, string ( string slice - &str), struct, enum, and vector are compound data types in Rust.

fn main() {
    // Array
    let i: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of i is: {:?}", i);
    println!("The value of i[0] is: {}", i[4]);

    // Tuple
    let tuple: (String, i32, f32, bool) = ("kumar".to_string(), 30, 4.5, false);
    println!("The value of tuple is: {:?}", tuple);

    let tuple_v2: (String, i32, f32, [i32; 5]) = ("kumar".to_string(), 30, 4.5, [1, 2, 3, 4, 5]);
    println!("The value of tuple v2 is: {:?}", tuple_v2);

    let tuple_v3: (&str, i32, f32, [i32; 5]) = ("kumar", 30, 4.5, [1, 2, 3, 4, 5]);
    println!("The value of tuple v3 is: {:?}", tuple_v3);

    let tuple_v4: (&String, i32, f32, [i32; 5]) = (&"kumar".to_string(), 30, 4.5, [1, 2, 3, 4, 5]);
    println!("The value of tuple v4 is: {:?}", tuple_v4);

    // Slice
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("The value of slice is: {:?}", slice);

    let animal_slice: &[&str] = &["lion", "tiger", "elephant", "giraffe"];
    println!("The value of Animal slice is: {:?}", animal_slice);

    let book_slice: &[&String] = &[
        &"book1".to_string(),
        &"book2".to_string(),
        &"book3".to_string(),
    ];
    println!("The value of Book slice is: {:?}", book_slice);

    // String
    let s1: String = "hello".to_string();
    println!("The value of s1 is: {}", s1);

    let mut s2: String = String::from("hello");
    println!("The value of s2 is: {}", s2);
    s2.push_str(", world!");
    println!("The value of s2 is: {}", s2);

    // Additional tests

    // Array
    let j: [char; 3] = ['a', 'b', 'c'];
    println!("The value of j is: {:?}", j);
    println!("The value of j[1] is: {}", j[1]);

    // Tuple
    let tuple_v5: (i32, f64, char) = (42, 3.14, 'x');
    println!("The value of tuple v5 is: {:?}", tuple_v5);

    // Slice
    let num_slice: &[i32] = &[10, 20, 30, 40, 50];
    println!("The value of num slice is: {:?}", num_slice);

    let str_slice: &[&str] = &["apple", "banana", "cherry"];
    println!("The value of str slice is: {:?}", str_slice);

    // String
    let s3: String = String::from("Rust");
    println!("The value of s3 is: {}", s3);

    let mut s4: String = "Programming".to_string();
    println!("The value of s4 is: {}", s4);
    s4.push('!');
    println!("The value of s4 is: {}", s4);
}
