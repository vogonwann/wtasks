// Tests for TaskManager

use tempfile::NamedTempFile;
use wtasks::TaskManager;

#[test]
fn test_task_manager_add_task() {
    let mut manager = TaskManager::new();
    manager.add_task("Test Task".to_string());
    assert_eq!(manager.tasks.len(), 1);
    assert_eq!(manager.tasks[0].lock().unwrap().name, "Test Task");
}

#[test]
fn test_task_manager_remove_task() {
    let mut manager = TaskManager::new();
    manager.add_task("Task 1".to_string());
    manager.add_task("Task 2".to_string());
    assert_eq!(manager.tasks.len(), 2);
    manager.remove_task(0);
    assert_eq!(manager.tasks.len(), 1);
    assert_eq!(manager.tasks[0].lock().unwrap().name, "Task 2");
}

#[test]
fn test_task_manager_mark_done() {
    let mut manager = TaskManager::new();
    manager.add_task("Test Task".to_string());
    assert_eq!(manager.tasks.len(), 1);
    manager.mark_done(1);
    assert_eq!(manager.tasks[0].lock().unwrap().done, true);
}

#[test]
fn test_task_manager_list_tasks() {
    let mut manager = TaskManager::new();
    manager.add_task("Task 1".to_string());
    manager.add_task("Task 2".to_string());
    manager.mark_done(1);
    let output = vec!["[x] 1: Task 1", "[ ] 2: Task 2"];
    let mut result = vec![];
    for (i, task) in manager.tasks.iter().enumerate() {
        let task = task.lock().unwrap();
        let status = if task.done { "[x]" } else { "[ ]" };
        result.push(format!("{} {}: {}", status, i + 1, task.name));
    }
    assert_eq!(result, output);
}

#[test]
fn test_task_manager_save_to_file() {
    let mut manager = TaskManager::new();
    manager.add_task("Test Task".to_string());
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();
    manager.save_to_file(file_path).unwrap();
    let loaded_manager = TaskManager::load_from_file(file_path);
    assert_eq!(loaded_manager.tasks.len(), 1);
    assert_eq!(loaded_manager.tasks[0].lock().unwrap().name, "Test Task");
}

#[test]
fn test_task_manager_load_from_file() {
    let mut manager = TaskManager::new();
    manager.add_task("Test Task".to_string());
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();
    manager.save_to_file(file_path).unwrap();
    let loaded_manager = TaskManager::load_from_file(file_path);
    assert_eq!(loaded_manager.tasks.len(), 1);
    assert_eq!(loaded_manager.tasks[0].lock().unwrap().name, "Test Task");
}

#[test]
fn test_task_manager_remove_task_out_of_bounds() {
    let mut manager = TaskManager::new();
    manager.add_task("Task 1".to_string());
    manager.add_task("Task 2".to_string());
    assert_eq!(manager.tasks.len(), 2);
    manager.remove_task(5);
    assert_eq!(manager.tasks.len(), 2);
}

#[test]
fn test_task_manager_mark_done_out_of_bounds() {
    let mut manager = TaskManager::new();
    manager.add_task("Task 1".to_string());
    assert_eq!(manager.tasks.len(), 1);
    manager.mark_done(5);
    assert_eq!(manager.tasks[0].lock().unwrap().done, false);
}

#[test]
fn test_task_manager_list_tasks_empty() {
    let manager = TaskManager::new();
    let output: Vec<String> = vec![];
    let mut result = vec![];
    for (i, task) in manager.tasks.iter().enumerate() {
        let task = task.lock().unwrap();
        let status = if task.done { "[x]" } else { "[ ]" };
        result.push(format!("{} {}: {}", status, i + 1, task.name));
    }
    assert_eq!(result, output);
}

#[test]
fn test_task_manager_list_tasks_multiple() {
    let mut manager = TaskManager::new();
    manager.add_task("Task 1".to_string());
    manager.add_task("Task 2".to_string());
    manager.mark_done(1);
    let output = vec!["[x] 1: Task 1", "[ ] 2: Task 2"];
    let mut result = vec![];
    for (i, task) in manager.tasks.iter().enumerate() {
        let task = task.lock().unwrap();
        let status = if task.done { "[x]" } else { "[ ]" };
        result.push(format!("{} {}: {}", status, i + 1, task.name));
    }
    assert_eq!(result, output);
}
