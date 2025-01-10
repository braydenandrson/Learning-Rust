// tests/integration_tests.rs
use data_pipeline::{
    processor::{DataProcessor, DataValidator, NumberDoubler},  // Added DataValidator
    pipeline::Pipeline,
    validators::NumberRangeValidator,
};

#[test]
fn test_complete_pipeline() {
    let mut pipeline = Pipeline::new(String::from("Integration Test Pipeline"));
    pipeline.add_processor(Box::new(NumberDoubler));
    
    let validator = NumberRangeValidator::new(0.0, 100.0);
    let input = 25.0;
    
    assert!(validator.validate(&input).is_ok());
    assert_eq!(pipeline.process(input).unwrap(), 50.0);
}