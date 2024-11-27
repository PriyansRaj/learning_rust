use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn main() {
    println!("Welcome to the file reader and writer!");
    loop {
        println!("Choose an option:");
        println!("1. Read from a file");
        println!("2. Write to a file");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => read_from_file(),
            "2" => write_to_file(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please enter 1, 2, or 3."),
        }
    }
}

fn read_from_file() {
    println!("Enter the name of the file to read: ");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");
    let file_name = file_name.trim();

    match fs::read_to_string(file_name) {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(e) => {
            println!("Failed to read file: {}", e);
        }
    }
}

fn write_to_file() {
    println!("Enter the name of the file to write to: ");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");
    let file_name = file_name.trim();

    println!("Enter the content to write: ");
    let mut content = String::new();
    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read input");

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open the file: {}", e);
            return;
        }
    };

    if let Err(e) = writeln!(file, "{}", content) {
        println!("Failed to write to file: {}", e);
    } else {
        println!("Content written successfully!");
    }
}
