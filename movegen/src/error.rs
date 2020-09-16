
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    details: String
}

impl Error {
    fn new(msg: &str) -> Error {
        Error{details: msg.to_string()}
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}
