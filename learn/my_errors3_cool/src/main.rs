use thiserror::Error;

#[derive(Debug, Error)]
enum CustomError {
    #[error("Error 1 with code {0}")]
    Error1(i32),
    #[error("Error 2 occurred!")]
    Error2,
    #[error("Error 3 with code {0}: {1}")]
    Error3(i32, String),
}

fn do_somethig() -> Result<(), CustomError> {
    Err(CustomError::Error1(101))
}

fn main() {
    if let Err(err) = do_somethig() {
        println!("{}", err);
    }
}
