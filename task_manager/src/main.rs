// src/main.rs
mod task;
mod storage;
mod display;

use task::{TaskPriority, TaskStatus};
use storage::TaskStorage;

fn main() {
    println!("Task Manager v1.0\n");
    
    // Create a new task storage
    let mut storage = TaskStorage::new();
    
    // Add several tasks with different priorities
    let rust_task = storage.add_task(
        String::from("Complete Rust project"),
        String::from("Finish the task manager implementation with tests"),
        TaskPriority::High,
    );
    
    let reading_task = storage.add_task(
        String::from("Read Chapter 9"),
        String::from("Study error handling in Rust"),
        TaskPriority::Medium,
    );

    let exercise_task = storage.add_task(
        String::from("Exercise"),
        String::from("Go for a 30-minute run"),
        TaskPriority::Low,
    );

    // Test adding tags
    println!("Adding tags to Rust project task...");
    if let Some(task) = storage.get_task_mut(rust_task) {
        task.add_tag(String::from("programming"));
        task.add_tag(String::from("rust"));
        task.add_tag(String::from("learning"));
        // Test adding duplicate tag (should not add)
        task.add_tag(String::from("rust"));
    }

    // Test status changes
    println!("Updating task statuses...");
    if let Some(task) = storage.get_task_mut(rust_task) {
        task.mark_in_progress();
        println!("Marked 'Complete Rust project' as in progress");
    }

    if let Some(task) = storage.get_task_mut(reading_task) {
        task.mark_completed();
        println!("Marked 'Read Chapter 9' as completed");
    }

    // Display all tasks
    println!("\nAll Tasks:");
    println!("----------");
    display::display_task_list(&storage.list_all_tasks());

    // Test task deletion
    println!("\nDeleting exercise task...");
    if storage.delete_task(exercise_task) {
        println!("Successfully deleted exercise task");
    } else {
        println!("Failed to delete task");
    }

    // Try to access deleted task (should be None)
    println!("\nTrying to access deleted task:");
    match storage.get_task(exercise_task) {
        Some(_) => println!("Task still exists!"),
        None => println!("Task not found (expected behavior)"),
    }

    // Test getting a single task
    println!("\nDetails of Rust project task:");
    if let Some(task) = storage.get_task(rust_task) {
        display::display_task(task);
    }

    // Try to get a task with invalid ID
    println!("\nTrying to access task with invalid ID (999):");
    match storage.get_task(999) {
        Some(task) => display::display_task(task),
        None => println!("Task not found (expected behavior)"),
    }

    // Display final state
    println!("\nFinal Task List:");
    println!("--------------");
    display::display_task_list(&storage.list_all_tasks());
}