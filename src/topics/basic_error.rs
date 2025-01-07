use std::fs::File;
// use std::io;
use std::io::ErrorKind;

pub fn error_handling() {
    let file = File::open("hello.txt");

    match file {
        Ok(_) => {
            println!("File opened successfully");
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("File not found: {:?}", error);
            }
            other_error => {
                panic!("An unexpected error occurred: {:?}", other_error);
            }
        },
    }
}
