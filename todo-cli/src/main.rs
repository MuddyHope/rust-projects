use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

const FILE: &str = "tasks.json";

#[derive(Parser)]
#[command(name = "tasker")]
#[command(version = "1.0")]
#[command(about = "Manage your tasks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        task: String,
    },
    List,
    Remove {
        #[arg(short, long)]
        index: usize,
    },
}

#[derive(Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<String>,
}

impl TaskList {
    fn load() -> Self {
        if let Ok(data) = fs::read_to_string(FILE) {
            serde_json::from_str(&data).unwrap_or(TaskList { tasks: vec![] })
        } else {
            TaskList { tasks: vec![] }
        }
    }

    fn save(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(FILE)
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

fn main() {
    let cli = Cli::parse();
    let mut task_list = TaskList::load();

    match cli.command {
        Commands::Add { task } => {
            task_list.tasks.push(task.clone());
            task_list.save();
            println!("✅ Task added: {}", task);
        }
        Commands::List => {
            if task_list.tasks.is_empty() {
                println!("📭 No tasks found.");
            } else {
                println!("📝 Tasks:");
                for (i, task) in task_list.tasks.iter().enumerate() {
                    println!("{}: {}", i, task);
                }
            }
        }
        Commands::Remove { index } => {
            if index < task_list.tasks.len() {
                let removed = task_list.tasks.remove(index);
                task_list.save();
                println!("🗑️ Removed: {}", removed);
            } else {
                println!("⚠️ Invalid index");
            }
        }
    }
}

