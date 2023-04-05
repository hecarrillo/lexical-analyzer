mod dynamic_dfa;
use std::io;

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
fn main() {
    println!("Enter a string to validate:");
    let input: String = read_user_input();
    if dynamic_dfa::validate(&input) == Ok(()) {
        print!("\nValidation Result:");
        println!(" Valid equation");
    } else {
        print!("\nValidation Result:");
        println!(" Invalid equation");
    }
}
