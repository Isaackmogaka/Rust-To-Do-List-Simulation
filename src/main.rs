use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use std::fs::File;
// use std::io::prelude::*;

// Task struct definition
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            done: false,
        }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}

// Function to add a task
fn add_task(tasks: &mut Vec<Task>, description: &str) {
    let task = Task::new(description);
    tasks.push(task);
}

// Function to view tasks
fn view_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        println!("{}: {} [{}]", index + 1, task.description, if task.done { "Done" } else { "Pending" });
    }
}

// Function to mark a task as done
fn mark_done(tasks: &mut Vec<Task>, task_index: usize) {
    if let Some(task) = tasks.get_mut(task_index) {
        task.mark_done();
    }
}

// Function to remove a task
fn remove_task(tasks: &mut Vec<Task>, task_index: usize) {
    if task_index < tasks.len() {
        tasks.remove(task_index);
    }
}

// Function to save tasks to a file
fn save_tasks(tasks: &Vec<Task>) {
    let file = File::create("tasks.json").unwrap();
    serde_json::to_writer(file, &tasks).unwrap();
}

// Function to load tasks from a file
fn load_tasks() -> Vec<Task> {
    let file = File::open("tasks.json");
    match file {
        Ok(file) => serde_json::from_reader(file).unwrap(),
        Err(_) => Vec::new(),
    }
}

fn main() {
    // Load tasks from file when the program starts
    let mut tasks = load_tasks();
    
    loop {
        println!("\nTo-Do List:");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Done");
        println!("4. Remove Task");
        println!("5. Quit");

        print!("Enter an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter task description: ");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                add_task(&mut tasks, description.trim());
            }
            2 => view_tasks(&tasks),
            3 => {
                println!("Enter task number to mark as done: ");
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num).unwrap();
                let task_num: usize = task_num.trim().parse().unwrap_or(0);
                mark_done(&mut tasks, task_num - 1);
            }
            4 => {
                println!("Enter task number to remove: ");
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num).unwrap();
                let task_num: usize = task_num.trim().parse().unwrap_or(0);
                remove_task(&mut tasks, task_num - 1);
            }
            5 => break,
            _ => println!("Invalid option, try again."),
        }

        // Save tasks to file after each change
        save_tasks(&tasks);
    }
}
