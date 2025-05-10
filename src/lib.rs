use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskSerializable {
    pub name: String,
    pub done: bool,
}

impl From<&Task> for TaskSerializable {
    fn from(task: &Task) -> Self {
        Self {
            name: task.name.clone(),
            done: task.done,
        }
    }
}

impl From<TaskSerializable> for Task {
    fn from(s: TaskSerializable) -> Self {
        Self {
            name: s.name,
            done: s.done,
        }
    }
}

pub type TaskRef = Arc<Mutex<Task>>;

#[derive(Debug)]
pub struct TaskManager {
    pub tasks: Vec<TaskRef>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn add_task(&mut self, name: String) {
        let task = Arc::new(Mutex::new(Task { name, done: false }));
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            println!("⚠️  No task with index {}.", index);
        }
    }

    pub fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            let task = task.lock().unwrap();
            let status = if task.done { "[x]" } else { "[ ]" };
            println!("{} {}: {}", status, i + 1, task.name);
        }
    }

    pub fn mark_done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get(index - 1) {
            let mut task = task.lock().unwrap();
            task.done = true;
        } else {
            println!("⚠️  No task with index {}.", index);
        }
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let serializable: Vec<TaskSerializable> = self
            .tasks
            .iter()
            .map(|task| {
                let t = task.lock().unwrap();
                TaskSerializable::from(&*t)
            })
            .collect();

        let json = serde_json::to_string_pretty(&serializable)?;
        fs::write(path, json)
    }

    pub fn load_from_file(path: &str) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(vec) = serde_json::from_str::<Vec<TaskSerializable>>(&content) {
                let tasks = vec
                    .into_iter()
                    .map(|t| Arc::new(Mutex::new(t.into())))
                    .collect();
                return Self { tasks };
            }
        }

        Self::new()
    }
}
