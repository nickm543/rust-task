mod task;

use std::{fs::File, io::Read, path::Path};

use crate::task::{Task, TaskList, Status};
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Name of task
        name: String,
        /// Description of task
        description: String
    },
    /// Remove a task
    Rm {
        /// Name of task
        name: String
    },
    /// Edit a task
    Edit {
        /// Name of task to edit
        name: String,
        /// New name of task
        new_name: String,
        /// New description of task
        new_description: String
    },
    /// List tasks
    Ls,
}

pub fn load_config(filename: &str) -> TaskList {
    let mut file = File::open(filename).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let new_task_list: TaskList = serde_json::from_str(&data).unwrap();
    return new_task_list;
}

fn main() {
    const CONFIG_FILE: &str = "./tasks.json";
    let cli = Cli::parse();
    let mut task_list: TaskList;
    
    // Check if config file exists; if so then load it
    if Path::new(CONFIG_FILE).exists() {
        println!("Trying to load config...");
        task_list = load_config(CONFIG_FILE);
    } else {
        task_list = TaskList::new();
    }

    match &cli.command {
        Some(Commands::Add { name, description }) => {
            // Add new task to task list
            let new_task = Task::new(
                String::from(name),
                String::from(description),
                Status::InProgress
            );

            task_list.add(new_task);
            task_list.write_config(CONFIG_FILE);
        }
        Some(Commands::Rm { name }) => {
            task_list.remove(name);
            task_list.write_config(CONFIG_FILE)
        }
        Some(Commands::Edit { name, new_name, new_description }) => {
            task_list.edit(name, new_name, new_description);
            task_list.write_config(CONFIG_FILE);
        }
        Some(Commands::Ls) => {
            task_list.display();
        }
        None => {
            println!("rust-task: Provide a subcommand.");
            println!("Try 'rust-task --help' for more information.");
        }
    }
}