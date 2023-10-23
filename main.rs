use std::io;

fn main() {
    // Define a list of strings
    let list = vec!["apple", "banana", "cherry", "date", "elderberry"];

    // Read user input
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the input to remove leading/trailing whitespaces and newline characters
    let input = input.trim();

    // Check if the input is in the list
    if list.contains(&input) {
        println!("It's in the list");
    } else {
        println!("Not in the list");
    }
}
