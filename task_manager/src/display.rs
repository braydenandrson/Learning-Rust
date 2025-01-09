// src/display.rs
use crate::task::{Task, TaskStatus, TaskPriority};

pub fn format_task(task: &Task) -> String {
    let status_str = match task.status {
        TaskStatus::Pending => "PENDING",
        TaskStatus::InProgress => "IN PROGRESS",
        TaskStatus::Completed => "COMPLETED",
    };

    let priority_str = match task.priority {
        TaskPriority::Low => "Low",
        TaskPriority::Medium => "Medium",
        TaskPriority::High => "High",
    };

    format!(
        "Task #{}: {} [{}] [Priority: {}]\n  Description: {}\n  Tags: {}\n",
        task.id,
        task.title,
        status_str,
        priority_str,
        task.description,
        if task.tags.is_empty() {
            "None".to_string()
        } else {
            task.tags.join(", ")
        }
    )
}

pub fn display_task(task: &Task) {
    println!("{}", format_task(task));
}

pub fn display_task_list(tasks: &[&Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    for task in tasks {
        display_task(task);
    }
}