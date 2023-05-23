/// This file contains all the custom errors that can be used.

#[derive(Debug)]
pub enum ErrorType {
    InvalidPort(String),
    InvalidIp(String),
    ParseError(String),
    NotImplemented(String),
}
impl ErrorType {
    pub fn message(&self)-> &String {
        match *self {
            ErrorType::InvalidPort(ref s) => s,
            ErrorType::InvalidIp(ref s) => s,
            ErrorType::ParseError(ref s) => s,
            ErrorType::NotImplemented(ref s) => s,
        }
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    NotImplemented(String),
}
impl DatabaseError {
    pub fn message(&self)-> &String {
        match *self {
            DatabaseError::ConnectionError(ref s) => s,
            DatabaseError::QueryError(ref s) => s,
            DatabaseError::NotImplemented(ref s) => s,
        }
    }
}
