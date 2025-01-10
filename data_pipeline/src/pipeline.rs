// src/pipeline.rs
use crate::error::ProcessError;
use crate::processor::DataProcessor;

pub struct Pipeline<T> {
    processors: Vec<Box<dyn DataProcessor<T, T>>>,  // Changed to use same type T
    name: String,
}

impl<T> Pipeline<T> {
    pub fn new(name: String) -> Self {
        Pipeline {
            processors: Vec::new(),
            name,
        }
    }

    pub fn add_processor(&mut self, processor: Box<dyn DataProcessor<T, T>>) {
        self.processors.push(processor);
    }

    pub fn process(&self, input: T) -> Result<T, ProcessError> {
        let mut current_data = input;
        
        for processor in &self.processors {
            current_data = processor.process(current_data)?;
        }
        
        Ok(current_data)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}