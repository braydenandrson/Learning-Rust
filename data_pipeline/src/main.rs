// src/main.rs
mod error;
mod processor;
mod pipeline;
mod validators;

use processor::{DataProcessor, DataValidator, NumberDoubler, UppercaseProcessor};  // Added DataValidator
use pipeline::Pipeline;
use validators::NumberRangeValidator;

fn main() {
    // Example 1: String processing pipeline
    println!("String Processing Pipeline:");
    let mut string_pipeline = Pipeline::new(String::from("String Processor"));
    string_pipeline.add_processor(Box::new(UppercaseProcessor));
    
    match string_pipeline.process(String::from("hello world")) {
        Ok(result) => println!("Processed result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Number processing pipeline
    println!("\nNumber Processing Pipeline:");
    let mut number_pipeline = Pipeline::new(String::from("Number Processor"));
    number_pipeline.add_processor(Box::new(NumberDoubler));
    
    let validator = NumberRangeValidator::new(0.0, 100.0);
    let input = 25.0;
    
    // Validate input
    if let Err(e) = validator.validate(&input) {
        println!("Validation error: {}", e);
        return;
    }
    
    match number_pipeline.process(input) {
        Ok(result) => println!("Processed result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}