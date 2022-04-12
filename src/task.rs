use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Task {
    name: String,
    class: String,
    date_due: String
}

pub struct TaskList {
    list: Vec<Task>
}

impl Task {
    pub fn new(name: String, class: String, date_due: String) -> Task {
        Task {
            name: name,
            class: class,
            date_due: date_due
        }
    }
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            list: vec![]
        }
    }
    pub fn add(&mut self, task: Task) {
        self.list.push(task);
    }
    pub fn remove(&mut self, name: &str) {
        let mut index = 0;

        // Get index of task specified by name
        for (i, t) in self.list.iter().enumerate() {
            if t.name == name {
                index = i;
            }
        }

        // Remove that task
        self.list.remove(index);
        println!("{} was removed.\n", name)
;    }
    pub fn display(&mut self) {
        let len = self.list.len();

        if len == 1 {
            println!("There is {} task.", len);
        } else {
            println!("There are {} tasks.", len);
        }

        for (i, t) in self.list.iter().enumerate() {
            println!("Task {}", i + 1);
            println!("\tName: {}", t.name);
            println!("\tClass: {}", t.class);
            println!("\tDate due: {}", t.date_due);
        }
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn load_config(&mut self, filename: &str) {
        // Read file line by line
        if let Ok(lines) = self.read_lines(filename) {
            for line in lines {
                if let Ok(current_line) = line {
                    // One task object per line
                    let split = current_line.split(":");
                    let tokens: Vec<&str> = split.collect();

                    let mut _name = String::new();
                    let mut _class = String::new();
                    let mut _date = String::new();

                    // Make sure proper fields are there
                    assert!(tokens[0] == "name", "Malformed config file, name field doesn't exist!");
                    assert!(tokens[2] == "class", "Malformed config file, class field doesn't exist!");
                    assert!(tokens[4] == "date", "Malformed config file, date field doesn't exist!");

                    _name = String::from(tokens[1]);
                    _class = String::from(tokens[3]);
                    _date = String::from(tokens[5]);

                    // Create the new task with those values
                    let new_task = Task::new(
                        _name,
                        _class,
                        _date
                    );

                    // Add the task to the list
                    self.add(new_task);
                }
            }
        }
    }
    pub fn write_config(&mut self, filename: &str) {
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(filename)
            .unwrap();

        // Create config file line by line from current task list
        for (_i, t) in self.list.iter().enumerate() {
            let config_line = format!("name:{}:class:{}:date:{}",
            t.name, t.class, t.date_due);

            if let Err(e) = writeln!(&mut f, "{}", config_line) {
                eprintln!("Failed to write to file: {}", e);
            }
        }
    }
}
