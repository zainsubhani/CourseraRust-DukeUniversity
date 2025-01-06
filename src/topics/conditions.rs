pub fn conditional_excercise() {
    let number = 6; // Declare a variable named `number` and assign it the value 6

    // If the value of the `number` variable is less than 5
    if number < 5 {
        println!("The number is less than 5"); // Print "The number is less than 5"
    } else if number == 5 {
        // If the value of the `number` variable is equal to 5
        // If the value of the `number` variable is not less than 5
        println!("The number is greater than or equal to 5"); // Print "The number is greater than or equal to 5"
    } else {
        // If the value of the `number` variable is not less than 5 and not equal to 5
        println!("The number is greater than 5"); // Print "The number is greater than 5"
    }
    if some(42) {
        println!("The number is 42");
    } else {
        println!("The number is not 42");
    }
    let maybe_number: Option<Option<()>> = None;
    let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("The number is {:?}", number);
    } else {
        println!("There is no number");
    }
}
