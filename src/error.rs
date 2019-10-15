use err_derive::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Error while opening serial port: {}", _0)]
    Serial(#[error(cause)] serial::Error),
    #[error(display = "Error communicating with serial port: {}", _0)]
    IO(#[error(cause)] std::io::Error),
    #[error(display = "Invalid data format : {}",_0)]
    Format(String),
}

impl From<serial::Error> for Error {
    fn from(err: serial::Error) -> Self {
        Error::Serial(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}



pub fn format_error(msg:String) -> Error {
    Error::Format(msg)
}
