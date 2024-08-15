mod task;

use std::{fs::File, io::{self, Write}, path::Path};

use crate::task::{Task, TaskList, Status, ConfigFile};
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
    },
    /// List tasks
    Ls,
}

fn main() {
    const CONFIG_FILE: &str = "./config.json";

    let config_file: ConfigFile;
    let mut tasks_filepath: &str = "./tasks.json";
    let mut task_list: TaskList = TaskList::new();
    let cli = Cli::parse();
    
    // Check if config file exists; if so then load it
    if Path::new(CONFIG_FILE).exists() {
        config_file = ConfigFile::load(CONFIG_FILE);
        tasks_filepath = config_file.tasks_filepath.as_str();
    } else {
        println!("{} Warning: No config file was found.", "[!]".yellow());
    }

    // Check if tasks file exists; if so then load it
    if Path::new(tasks_filepath).exists() {
        task_list = task_list.load(tasks_filepath);
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
            task_list.write_tasks_file(tasks_filepath);
        }
        Some(Commands::Rm { name }) => {
            task_list.remove(name);
            task_list.write_tasks_file(tasks_filepath)
        }
        Some(Commands::Edit { name }) => {
            let mut new_name = String::new();
            let mut new_desc = String::new();
            let mut new_status: Status = Status::NotStarted;

            println!("{} Editing task '{}':", "[*]".yellow(), name);

            print!("\tEnter new name: ");
            io::stdout().flush().expect("Failed to flush stdout.");
            io::stdin().read_line(&mut new_name).expect("Failed to read new task name");

            print!("\tEnter new description: ");
            io::stdout().flush().expect("Failed to flush stdout.");
            io::stdin().read_line(&mut new_desc).expect("Failed to read new task description");

            task_list.edit(name, new_name.trim(), new_desc.trim(), new_status); 
            task_list.write_tasks_file(tasks_filepath);
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