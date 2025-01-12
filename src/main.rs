mod topics {
    pub mod control_flow;
    pub mod matchex;
    pub mod variable; // Declare the `variable` module inside the `Topics` module
                      // Declare the `control_flow` module inside the `Topics` module
    pub mod FilesizeConverter;
    pub mod basic_error;
    pub mod return_value;
}

fn main() {
    println!("Hello, world!");
    topics::basic_error::error_handling(); // Call the `error_handling` function from the `basic_error` module
    let input: String = String::from("apple,banana,carrot");

    let delimiter = ',';
    let field_number = 2;
    let field = topics::return_value::split_string(input, delimiter, field_number); // Call the `return_value_excercise` function from the `return_value` module
    println!("Extracted field: {}", field); // Display the extracted field

    // topics::matchex::match_excercise(); // Call the `match_excercise` function from the `matchex` module
    // println!("Hello, world!");
    // println!("Welcome to the Rust programming language! Variable Topics");
    // topics::variable::variable_excercise(); // Call the `greet` function from the `variable` module
    // println!("Control Flow Topics");
    // topics::control_flow::control_flow_excercise(); // Call the `control_flow_excercise` function from the `control_flow` module
}
