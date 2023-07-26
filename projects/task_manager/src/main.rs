use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use chrono::{DateTime, Local };
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
enum Property {
    Low,
    Medium,
    High,
}

impl Property {
    fn to_string(&self) -> String {
        match self {
            Property::Low => "Low".to_owned(),
            Property::Medium => "Medium".to_owned(),
            Property::High => "High".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    priority: Property,
    add_time: DateTime<Local>, // время добавления
}

impl Task {
    fn new(name: String, description: String, priority: Property) -> Self {
        Self {
            name,
            description,
            priority,
            add_time: Local::now(),
        }
    }

    fn new_from_console() -> Self {
        let name = ConsoleManager::input("Enter new task name: ").unwrap();
        let description = ConsoleManager::input("Enter new task description: ").unwrap();
        let priority = match ConsoleManager::input("Enter new task priority: ")
            .unwrap()
            .to_lowercase()
            .as_str()
        {
            "low" => Property::Low,
            "medium" => Property::Medium,
            "high" => Property::High,
            _ => {
                println!("Invalid priority, setting to low");
                Property::Low
            }
        };
        Self::new(name, description, priority)
    }

    fn print_task(&self) {
        println!(
            "\n{} | {} | {}\n\"{}\"",
            self.name,
            self.priority.to_string(),
            self.add_time.format("%d-%m-%Y %H:%M:%S"),
            self.description
        );
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|t| t.name == name)
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Task \"{}\" removed successfully", name))
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }
    }

    fn edit_task(&mut self, name: &str, update_task: Task) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            match self.tasks.get_mut(index) {
                Some(task) => {
                    task.name = update_task.name;
                    task.description = update_task.description;
                    task.priority = update_task.priority;
                    Ok(format!("Task \"{}\" updated successfully", name))
                }
                None => Err("Erroe borrowing task".to_owned()),
            }
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }
    }

    fn save_to_file(&self, filename: &str) -> Result<String, String> {
        if !Path::new(filename).exists() {
            let file = match File::create(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file \"{filename}\": {err}"))
            };
            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Data saved successfully!".to_owned()),
                Err(err) => Err(format!("Error saving data to file \"{filename}\": {err}")),
            }
        }
        else {
            Err("File \"{filename}\" already exists!".to_owned())
        }
    }

    fn read_from_file(&mut self, filename: &str) -> Result<String, String> {
        if Path::new(filename).exists() {
            let file = match File::open(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {}", err))
            };
            let reader = BufReader::new(file);
            self.tasks = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(err) => {
                    return Err(format!("Error reading data from file: {}", err));
                }
            };
            Ok("Data readed successfully!".to_owned())
        } else {
            Err(format!("File \"{}\" doesn't exist!", filename))
        }
    }
}

struct ConsoleManager {
    tasks_manager: TaskManager,
    menu_options: Vec<String>
}

impl ConsoleManager {
    fn new() -> Self {
        Self {
            tasks_manager: TaskManager::new(),
            menu_options: vec![
                "Add task".to_owned(),
                "Find task".to_owned(),
                "Edit task".to_owned(),
                "Remove task".to_owned(),
                "Print tasks".to_owned(),
                "Save tasks to file".to_owned(),
                "Load tasks from file".to_owned(),
            ]
        }
    }

    fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, menu_option);
        }
    }

    fn input(query: &str) -> std::io::Result<String> {
        print!("{}", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn process_command(&mut self) {
        match Self::input("\nEnter command index: ") {
            Ok(commnd) => {
                match commnd.as_str() {
                    "1" => {
                        self.tasks_manager.add_task(Task::new_from_console());
                    },

                    "2" => {
                        let name = match Self::input("Enter task name to find: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {err}");
                                return;
                            }
                        };
                        match self.tasks_manager.find_task(name.as_str()) {
                            Some(index) => {
                                println!("Task found!");
                                self.tasks_manager.tasks.get(index).unwrap().print_task();
                            },
                            None => println!("Task with name \"{}\" doesn't exist", name)
                        }
                    },

                    "3" => {
                        let name = match Self::input("Enter task name to edit: ")
                        {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {err}");
                                return;
                            }
                        };
                        match self.tasks_manager.edit_task(name.as_str(), Task::new_from_console()) {
                            Ok(msg) => println!("{}", msg),
                            Err(err) => println!("Error editing task: {err}"),
                        }
                    },

                    "4" => {
                        let name = match Self::input("Enter task name to remove: ")
                        {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {err}");
                                return;
                            }
                        };
                        match self.tasks_manager.remove_task(name.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(err) => println!("Error removing task: {err}"),
                        }
                    },

                    "5" => self.tasks_manager.print_tasks(),

                    "6" => {
                        let mut filename = match Self::input("Enter file name to save data in: ")
                        {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting user input: {err}");
                                return;
                            }
                        };
                        if !filename.contains(".json") {
                            filename.push_str(".json");
                        }
                        match self.tasks_manager.save_to_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "7" => {
                         let mut filename = match Self::input("Enter file name to read data from: ")
                        {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting user input: {err}");
                                return;
                            }
                        };
                        if !filename.contains(".json") {
                            filename.push_str(".json");
                        }
                        match self.tasks_manager.read_from_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    _ => println!("I don't understand this command"),
                }
            }
            Err(err) => { println!("Error getting user input: {err}") }
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
    }
}
