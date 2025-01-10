// src/lib.rs
pub mod error;
pub mod processor;
pub mod pipeline;
pub mod validators;

#[cfg(test)]
mod tests {
    use super::*;
    use processor::{DataProcessor, DataValidator, NumberDoubler, UppercaseProcessor};  // Added DataValidator
    use pipeline::Pipeline;
    use validators::NumberRangeValidator;

    #[test]
    fn test_uppercase_processor() {
        let processor = UppercaseProcessor;
        let result = processor.process(String::from("hello"));
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn test_number_doubler() {
        let processor = NumberDoubler;
        let result = processor.process(5.0);
        assert_eq!(result.unwrap(), 10.0);
    }

    #[test]
    fn test_number_validator() {
        let validator = NumberRangeValidator::new(0.0, 10.0);
        assert!(validator.validate(&5.0).is_ok());
        assert!(validator.validate(&15.0).is_err());
    }

    #[test]
    fn test_pipeline() {
        let mut pipeline = Pipeline::new(String::from("Test Pipeline"));
        pipeline.add_processor(Box::new(NumberDoubler));
        pipeline.add_processor(Box::new(NumberDoubler));
        
        let result = pipeline.process(5.0);
        assert_eq!(result.unwrap(), 20.0); // 5.0 -> 10.0 -> 20.0
    }
}