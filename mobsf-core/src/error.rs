use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use crate::error::Cause::{HttpClientError, IoError};

#[derive(Debug)]
pub enum Cause {
    HttpClientError,
    IoError,
    InvalidHttpResponse(u16),
}

pub struct AppError {
    pub cause: Cause,
    pub message: String,
}

impl Error for AppError {}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "✖ {:?}, {}", self.cause, self.message)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.cause, self.message)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError {
            cause: HttpClientError,
            message: e.to_string(),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError {
            cause: IoError,
            message: e.to_string(),
        }
    }
}