pub fn error_handling() {
    let file = file::open("hello.txt");
    match file {
        Ok(file) => {
            println!("File opened successfully");
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found");
            }
            other_error => {
                println!("Error: {:?}", other_error);
            }
        },
    }
}
