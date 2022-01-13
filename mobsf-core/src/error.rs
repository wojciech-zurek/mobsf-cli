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

pub struct MobsfError {
    pub cause: Cause,
    pub message: String,
}

impl Error for MobsfError {}

impl Debug for MobsfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.cause, self.message)
    }
}

impl Display for MobsfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.cause, self.message)
    }
}

impl From<reqwest::Error> for MobsfError {
    fn from(e: reqwest::Error) -> Self {
        MobsfError {
            cause: HttpClientError,
            message: e.to_string(),
        }
    }
}

impl From<io::Error> for MobsfError {
    fn from(e: io::Error) -> Self {
        MobsfError {
            cause: IoError,
            message: e.to_string(),
        }
    }
}