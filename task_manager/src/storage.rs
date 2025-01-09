// src/storage.rs
use std::collections::HashMap;
use crate::task::Task;

pub struct TaskStorage {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskStorage {
    pub fn new() -> TaskStorage {
        TaskStorage {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: String, description: String, priority: crate::task::TaskPriority) -> u32 {
        let id = self.next_id;
        let task = Task::new(id, title, description, priority);
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    pub fn list_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn delete_task(&mut self, id: u32) -> bool {
        self.tasks.remove(&id).is_some()
    }
}