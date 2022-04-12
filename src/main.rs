mod task;

use clap::{arg, command, Command};

use crate::task::{Task, TaskList};

const FILENAME: &str = "/home/nick/.config/rust-task/config";

fn main() {
    // Parse arguments
    let matches = command!()
        .subcommand(
            Command::new("new")
                .about("Create a new task")
                .arg(arg!(
                    --name <VALUE> "Name for the new task"
                ).required(true))
                .arg(arg!(
                    --class <VALUE> "Class for the new task"
                ).required(false))
                .arg(arg!(
                    --datedue <VALUE> "Date the task is due"
                ).required(false))
        )
        .subcommand(
            Command::new("list")
                .about("List tasks")
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a task")
                .arg(arg!(
                    --name <VALUE> "Task to be removed"
                ).required(true))
        )
        .get_matches();

    // Create the task list
    let mut task_list = TaskList::new();

    // Eventually use the crate 'dirs' to get home directory automatically instead of hard coding
    task_list.load_config(FILENAME);

    if let Some(_matches) = matches.subcommand_matches("list") {
        // Display the task list
        task_list.display();
    }
    if let Some(matches) = matches.subcommand_matches("new") {
        // Add a new task and save it to the config file
        let _name = matches.value_of("name").unwrap();
        let _class = matches.value_of("class").unwrap();
        let _date = matches.value_of("datedue").unwrap();

        let new_task = Task::new(
            String::from(_name),
            String::from(_class),
            String::from(_date)
        );

        task_list.add(new_task);
        task_list.write_config(FILENAME);
    }
    if let Some(matches) = matches.subcommand_matches("rm") {
        let _name = matches.value_of("name").unwrap();

        // Remove the task and write the changes to the config
        task_list.remove(_name);
        task_list.write_config(FILENAME);
    }
}
