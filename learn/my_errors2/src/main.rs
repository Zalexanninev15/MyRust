use std::error::Error;
use std::fmt::{ Display, Formatter };

#[derive(Debug)]
enum CustomError {
    Error1,
    Error2(i32),
    Error3(i32, String)
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self
        {
            Self::Error1 => write!(f, "Custom Error1 occurred!"),
            Self::Error2(code) => write!(f, "Custom Error2 occurred! Code: {}", code),
            Self::Error3(code, message) => write!(f, "Custom Error3 occurred! Code: {}, Message: {}", code, message),
        }
    }
}

impl Error for CustomError {}

fn do_something() -> Result<(), CustomError> {
    // CODE
    // Err(CustomError::Error1)
    // Err(CustomError::Error2(32))
    Err(CustomError::Error3(534, "Test error".to_owned()))
}

fn main() {
    match do_something() {
        Ok(_) => {}
        Err(err) => { println!("{}", err) }
    }
}
