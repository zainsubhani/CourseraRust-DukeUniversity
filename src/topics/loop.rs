pub fn loop_excercise() {
    let mut counter = 0; // Declare a mutable variable named `counter` and assign it the value 0

    // Loop until the value of the `counter` variable is equal to 5
    loop {
        println!("The value of the counter is: {}", counter); // Print the value of the `counter` variable
        counter += 1; // Increment the value of the `counter` variable by 1

        // If the value of the `counter` variable is equal to 5
        if counter == 5 {
            break; // Exit the loop
        }
    }
    for i in 0..5 {
        println!("The value of i is: {}", i);
    }
} // End of the `loop_excercise` function
  // End of the `loop` module
  // do while loop

// break and continue
