// src/processor.rs
use crate::error::ProcessError;

pub trait DataProcessor<T, U> {
    fn process(&self, input: T) -> Result<U, ProcessError>;
    fn name(&self) -> &str;
}

pub trait DataValidator<T> {
    fn validate(&self, data: &T) -> Result<(), ProcessError>;
}

// Example processor that converts strings to uppercase
pub struct UppercaseProcessor;

impl DataProcessor<String, String> for UppercaseProcessor {
    fn process(&self, input: String) -> Result<String, ProcessError> {
        Ok(input.to_uppercase())
    }

    fn name(&self) -> &str {
        "Uppercase Processor"
    }
}

// Number doubling processor
pub struct NumberDoubler;

impl DataProcessor<f64, f64> for NumberDoubler {
    fn process(&self, input: f64) -> Result<f64, ProcessError> {
        Ok(input * 2.0)
    }

    fn name(&self) -> &str {
        "Number Doubler"
    }
}