use std::{fs::{write, File, OpenOptions}, io::Read};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Complete
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    name: String,
    description: String,
    status: Status
}

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    list: Vec<Task>
}

impl Task {
    pub fn new(name: String, description: String, status: Status) -> Task {
        Task {
            name,
            description,
            status
        }
    }
}

impl TaskList {
    // Create new task list
    pub fn new() -> TaskList {
        TaskList {
            list: vec![]
        }
    }

    // Add task to list
    pub fn add(&mut self, task: Task) {
        self.list.push(task);
    }

    // Remove task from list
    pub fn remove(&mut self, name: &str) {
        let index = self.list.iter().position(|r| r.name.eq(name)).unwrap();

        self.list.remove(index);
        println!("{} was removed.", name)
    }
    
    pub fn display(&mut self) {
        let len = self.list.len();

        if len == 1 {
            println!("There is 1 task.");
        } else {
            println!("There are {} tasks.", len);
        }

        for (i, t) in self.list.iter().enumerate() {
            println!("{} {}", "Task #".green().bold(), i + 1);
            println!("\t{}: {}", "Name".yellow().bold(), t.name);
            println!("\t{}: {}", "Description".yellow().bold(), t.description);
            println!("\t{}: {:?}", "Status".yellow().bold(), t.status);
        }
    }


    pub fn write_config(&mut self, filename: &str) {
        // Serialize task list to JSON
        let json = serde_json::to_string(&self).unwrap(); 

        // Write to file
        match write(filename, json) {
            Ok(_) => println!("\n{} Wrote file '{}' containing current task list", "[*]".green(), filename),
            Err(e) => eprintln!("{} Failed to write to file '{}': {}", "[!]".red(), filename, e),
        };

    }
}