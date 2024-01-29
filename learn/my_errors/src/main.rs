use std::error::Error;
use std::fmt::{ Display, Formatter };

#[derive(Debug)]
struct CustomError {
    message: String
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Custom Error occurred!")
    }
}

impl Error for CustomError {}

fn do_something() -> Result<(), CustomError> {
    // CODE
    Err(CustomError { message: "Error code 379!".to_owned() })
}

fn main() {
    match do_something() {
        Ok(_) => {}
        Err(err) => { println!("{} - {}", err, err.message) }
    }
}
