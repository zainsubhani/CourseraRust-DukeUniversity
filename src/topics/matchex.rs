use std::io;

pub fn match_excercise() {
    println!("Please enter the option");
    let mut _name = String::new();
    io::stdin()
        .read_line(&mut _name)
        .expect("Failed to read line");
    let _name = _name.trim();

    // Declare a variable named `name` and assign it the value "Rustacean"
    match _name {
        "rustacean" => println!("Hello, {}!", _name), // If the value of the `name` variable is "Rustacean", print "Hello, Rustacean!"
        "zain" => println!("Hello, world! {}", _name), // If the value of the `name` variable is not "Rustacean", print "Hello, world!"
        "rust " => println!("Welcome to the Rust programming language! Variable Topics"),
        _ => println!("exit "), // If the value of the `name` variable is not "Rustacean" or "zain", print "Welcome to the Rust programming language! Variable Topics"
    }
}

// Declare the `match_excercise` function inside the `matchex` module
// End of the `matchex` module
// case sensitive
