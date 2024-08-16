use std::{ fs::{write, File }, io::Read, process };
use colored::Colorize;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    NotStarted = 0,
    InProgress = 1,
    Complete = 2
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

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub tasks_filepath: String
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

    // Edit task
    pub fn edit(&mut self, name: &str, new_name: &str, new_desc: &str, new_status: Status) {
        let index = self.list.iter().position(|r| r.name.eq(name));

        if index.is_none() {
            eprintln!("{} No such task with name '{}'", "[!]".red(), name);
            process::exit(1);
        }

        self.list[index.unwrap()].name = String::from(new_name);
        self.list[index.unwrap()].description = String::from(new_desc);
        self.list[index.unwrap()].status = new_status;
    }

    // Remove task from list
    pub fn remove(&mut self, name: &str) {
        let index = self.list.iter().position(|r| r.name.eq(name));

        if index.is_none() {
            eprintln!("{} No such task with name '{}'", "[!]".red(), name);
            process::exit(1);
        }

        self.list.remove(index.unwrap());
        println!("Task '{}' was removed.", name)
    }
    
    pub fn display(&mut self) {
        let len = self.list.len();

        if len == 1 {
            println!("{} There is 1 task.", "[*]".green());
        } else {
            println!("{} There are {} tasks.", "[*]".green(), len);
        }

        for (i, t) in self.list.iter().enumerate() {
            println!("{} {}", "Task #".green().bold(), i + 1);
            println!("\t{}: {}", "Name".yellow().bold(), t.name);
            println!("\t{}: {}", "Description".yellow().bold(), t.description);
            println!("\t{}: {:?}", "Status".yellow().bold(), t.status);
        }
    }

    pub fn load(&mut self, filename: &str) -> TaskList {
        let file = File::open(filename);

        if file.is_err() {
            eprintln!("{} Failed to open tasks file '{}'", "[!]".red(), filename);
            process::exit(1);
        }

        let mut data = String::new();
        file.unwrap().read_to_string(&mut data).unwrap();

        let new_task_list: TaskList = serde_json::from_str(&data).unwrap();
        return new_task_list;
    }

    pub fn write_tasks_file(&mut self, filename: &str) {
        // Serialize task list to JSON
        let json = serde_json::to_string(&self); 

        if json.is_err() {
            eprintln!("{} Failed to serialize JSON for tasks file", "[!]".red());
            process::exit(1);
        }

        // Write to file
        match write(filename, format!("{}\n", json.unwrap())) {
            Ok(_) => println!("{} Wrote tasks file '{}'", "[*]".green(), filename),
            Err(e) => eprintln!("{} Failed to write to file '{}': {}", "[!]".red(), filename, e),
        };

    }
}

impl ConfigFile {
    pub fn load(filename: &str) -> ConfigFile {
        let file = File::open(filename);

        if file.is_err() {
            eprintln!("{} Failed to open configuration file '{}'", "[!]".red(), filename);
            process::exit(1);
        }

        let mut data = String::new();
        file.unwrap().read_to_string(&mut data).unwrap();

        let config_file: ConfigFile = if let Ok(config) = serde_json::from_str(&data) {
            config
        } else {
            eprintln!("{} Failed to deserialize JSON configuration file '{}'", "[!]".red(), filename);
            process::exit(1);
        };

        return config_file;
    }
}