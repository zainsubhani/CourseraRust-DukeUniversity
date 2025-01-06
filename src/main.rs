mod topics {
    pub mod control_flow;
    pub mod matchex;
    pub mod variable; // Declare the `variable` module inside the `Topics` module
                      // Declare the `control_flow` module inside the `Topics` module
}

fn main() {
    topics::matchex::match_excercise(); // Call the `match_excercise` function from the `matchex` module
    println!("Hello, world!");
    println!("Welcome to the Rust programming language! Variable Topics");
    topics::variable::variable_excercise(); // Call the `greet` function from the `variable` module
    println!("Control Flow Topics");
    topics::control_flow::control_flow_excercise(); // Call the `control_flow_excercise` function from the `control_flow` module
}
