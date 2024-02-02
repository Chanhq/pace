
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not resolve index")]
    IndexError(String),

    #[error("Invalid value supplied")]
    ValueError(String),

    #[error("Something is wrong with the specified filepath")]
    FileError(#[from] std::io::Error),

    #[error("Something is wrong with the specified filepath")]
    ParseError(String),
}