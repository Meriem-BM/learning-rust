use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    title: String,
    done: bool,
}

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        let tasks = TaskManager::load_tasks().unwrap_or_else(|_| Vec::new());
        TaskManager { tasks }
    }

    pub fn add_task(&mut self, title: String) {
        let task = Task {
            id: self.next_id(),
            title: title.trim().to_string(),
            done: false,
        };
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: u32) {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
        } else {
            println!("❌ Task with ID {} not found", id);
        }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_task(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn save_tasks(&self) -> io::Result<()> {
        let file = File::create("tasks.json")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.tasks)?;
        Ok(())
    }

    pub fn load_tasks() -> io::Result<Vec<Task>> {
        if !Path::new("tasks.json").exists() {
            return Ok(Vec::new());
        }

        let file = File::open("tasks.json")?;
        let reader = BufReader::new(file);
        let tasks = serde_json::from_reader(reader)?;
        Ok(tasks)
    }

    fn next_id(&self) -> u32 {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }
}

impl Task {
    pub fn mark_as_done(&mut self) {
        self.done = true;
    }

    pub fn mark_as_not_done(&mut self) {
        self.done = false;
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    println!("Enter a task:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    task_manager.add_task(title);
    task_manager.save_tasks().unwrap();

    println!("\n📋 All Tasks:");
    for task in task_manager.get_tasks() {
        println!(
            "[{}] {} - {}",
            task.id,
            if task.done { "✓" } else { " " },
            task.title
        );
    }

    println!("\nEnter the ID of the task to mark as done:");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).unwrap();

    if let Ok(id) = id_input.trim().parse::<u32>() {
        match task_manager.get_task(id) {
            Some(task) => {
                task.mark_as_done();
                println!("✅ Task marked as done.");
            }
            None => println!("❌ Task with ID {} not found", id),
        }
    } else {
        println!("⚠️ Invalid ID entered.");
    }

    task_manager.save_tasks().unwrap();

    println!("\n📋 Updated Tasks:");
    for task in task_manager.get_tasks() {
        println!(
            "[{}] {} - {}",
            task.id,
            if task.done { "✓" } else { " " },
            task.title
        );
    }
}
