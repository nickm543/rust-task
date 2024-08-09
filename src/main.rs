mod task;

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
        /// Name of task
        name: String
    },
    /// List tasks
    Ls,
}

fn main() {
    let cli = Cli::parse();    

    let mut task_list = TaskList::new();

    match &cli.command {
        Some(Commands::Add { name, description }) => {
            // Add new task to task list
            let new_task = Task::new(
                String::from(name),
                String::from(description),
                Status::InProgress
            );

            task_list.add(new_task);
            // task_list.display();
            task_list.write_config("./tasks.json");
        }
        Some(Commands::Rm { name }) => {
            println!("rm command used with argument {}", name);
        }
        Some(Commands::Edit { name }) => {
            println!("edit command used with argument {}", name);
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