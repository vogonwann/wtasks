use clap::Parser;
use wtasks::TaskManager;

#[derive(Parser, Debug)]
#[command(name = "task-runner")]
#[command(about = "Burketov async task processor", version, long_about = None)]
struct Args {
    /// Add a new task
    #[arg(short, long, value_name = "TEXT")]
    name: Option<String>,

    /// List all tasks
    #[arg(short, long)]
    list: bool,

    /// Mark a task as done (1-based index)
    #[arg(long, value_name = "INDEX")]
    done: Option<usize>,

    /// Remove a task (1-based index)
    #[arg(short, long, value_name = "INDEX")]
    remove: Option<usize>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut manager = TaskManager::load_from_file("tasks.json");

    if args.list {
        println!("\n📋 Task List:");
        manager.list_tasks();
    }

    if let Some(index) = args.done {
        manager.mark_done(index);
        let _ = manager.save_to_file("tasks.json");
    }

    if let Some(task_name) = args.name {
        manager.add_task(task_name);
        let _ = manager.save_to_file("tasks.json");
    }

    if let Some(index) = args.remove {
        manager.remove_task(index - 1);
        let _ = manager.save_to_file("tasks.json");
    }

    println!("\n📋 Task List:");
    manager.list_tasks();
}
