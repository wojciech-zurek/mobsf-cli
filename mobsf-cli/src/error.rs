use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use mobsf_core::error::MobsfError;

pub struct AppError {
    pub message: String,
}

impl Error for AppError {}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<MobsfError> for AppError {
    fn from(e: MobsfError) -> Self {
        AppError {
            message: e.to_string()
        }
    }
}