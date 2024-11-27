use crate::task::Task;
use std::fs;

pub fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write("tasks.json", json).expect("Failed to save tasks");
    println!("Tasks saved successfully!");
}

pub fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}
