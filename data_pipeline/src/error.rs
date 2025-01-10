// src/error.rs
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ProcessError {
    ValidationError(String),
    TransformError(String),
    InputError(String),
    OutputError(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ProcessError::TransformError(msg) => write!(f, "Transform error: {}", msg),
            ProcessError::InputError(msg) => write!(f, "Input error: {}", msg),
            ProcessError::OutputError(msg) => write!(f, "Output error: {}", msg),
        }
    }
}

impl Error for ProcessError {}