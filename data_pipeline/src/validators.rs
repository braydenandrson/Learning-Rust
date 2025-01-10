// src/validators.rs
use crate::error::ProcessError;
use crate::processor::DataValidator;

pub struct NumberRangeValidator {
    min: f64,
    max: f64,
}

impl NumberRangeValidator {
    pub fn new(min: f64, max: f64) -> Self {
        NumberRangeValidator { min, max }
    }
}

impl DataValidator<f64> for NumberRangeValidator {
    fn validate(&self, data: &f64) -> Result<(), ProcessError> {
        if *data >= self.min && *data <= self.max {
            Ok(())
        } else {
            Err(ProcessError::ValidationError(format!(
                "Number {} is outside valid range [{}, {}]",
                data, self.min, self.max
            )))
        }
    }
}