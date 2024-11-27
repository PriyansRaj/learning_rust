mod storage;
mod task;

use std::io;
use storage::{load_tasks, save_tasks};
use task::{add_task, delete_task, mark_task_completed, view_tasks};

fn main() {
    let mut tasks = load_tasks();
    loop {
        println!("\nTask Manager:");
        println!("1. Add a new task");
        println!("2. View all tasks");
        println!("3. Mark a task as completed");
        println!("4. Delete a task");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("GoodBye");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
