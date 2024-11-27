use std::io;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

pub fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter the task description: ");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");

    let id = if tasks.is_empty() {
        1
    } else {
        tasks.last().unwrap().id + 1
    };

    tasks.push(Task {
        id,
        description: description.trim().to_string(),
        completed: false,
    });
    println!("Task added successfully!");
}

pub fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    println!("\nTasks:");
    for task in tasks {
        println!(
            "[{}] {}: {}",
            if task.completed { "X" } else { " " },
            task.id,
            task.description
        );
    }
}

pub fn mark_task_completed(tasks: &mut Vec<Task>) {
    println!("Enter the task ID to mark as completed: ");
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read input");

    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid task ID.");
            return;
        }
    };

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        println!("Task marked as completed!");
    } else {
        println!("Task not found.");
    }
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    println!("Enter the task ID to delete: ");
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read input");

    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid task ID.");
            return;
        }
    };

    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        println!("Task deleted successfully!");
    } else {
        println!("Task not found.");
    }
}
