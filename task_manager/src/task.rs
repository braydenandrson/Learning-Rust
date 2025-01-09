// src/task.rs
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub created_at: u64,
    pub tags: Vec<String>,
}

impl Task {
    pub fn new(id: u32, title: String, description: String, priority: TaskPriority) -> Task {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Pending,
            priority,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            tags: Vec::new(),
        }
    }

    pub fn mark_in_progress(&mut self) {
        self.status = TaskStatus::InProgress;
    }

    pub fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}