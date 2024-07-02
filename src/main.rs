use clap::Parser;
use directories::ProjectDirs;
use owo_colors::OwoColorize;
use std::{fs, path::PathBuf};
use tasks_rs::{AddArgs, EditArgs, GetArgs, RemoveArgs, Task, TaskManager, TaskStatus};

#[derive(Parser)]
#[command(version, about)]
enum Cli {
    /// Get all tasks
    Get(GetArgs),
    /// Add a new task
    Add(AddArgs),
    /// Edit an existing task
    Edit(EditArgs),
    /// Remove an existing task
    Remove(RemoveArgs),
}

fn main() {
    let cli = Cli::parse();
    let data_dir = get_data_dir();
    let path = data_dir.join("tasks.json");
    let manager = TaskManager::load(path);
    match cli {
        Cli::Get(args) => get_tasks(manager, args),
        Cli::Add(args) => add_task(manager, args),
        Cli::Edit(args) => edit_task(manager, args),
        Cli::Remove(args) => remove_task(manager, args),
    }
}

fn get_tasks(manager: TaskManager, args: GetArgs) {
    let tasks = manager.get(args);
    if let Some(status) = args.status {
        print_tasks(&tasks, status);
    } else {
        print_tasks(&tasks, TaskStatus::ToDo);
        println!();
        print_tasks(&tasks, TaskStatus::InProgress);
        println!();
        print_tasks(&tasks, TaskStatus::Complete);
    }
    println!();
}

fn add_task(mut manager: TaskManager, args: AddArgs) {
    let task = manager.add(args);
    println!("{}\n{task}", task.status.bold().underline().yellow());
}

fn edit_task(mut manager: TaskManager, args: EditArgs) {
    match manager.edit(args) {
        Ok(task) => println!("{}\n{task}", task.status.bold().underline().yellow()),
        Err(error) => eprintln!("{}: {error}", "Error".red()),
    }
}

fn remove_task(mut manager: TaskManager, args: RemoveArgs) {
    if let Err(error) = manager.remove(args) {
        eprintln!("{}: {error}", "Error".red());
    } else {
        println!("{}", "Task deleted".bright_red());
    }
}

fn get_data_dir() -> PathBuf {
    let project_dirs =
        ProjectDirs::from("com", "Gamertike", "tasks-rs").expect("Unable to get home directory");
    let data_dir = project_dirs.data_dir();
    fs::create_dir_all(data_dir).expect("Unable to create data directory");
    data_dir.to_path_buf()
}

fn print_tasks(tasks: &[&Task], status: TaskStatus) {
    let mut tasks = tasks.iter().filter(|task| task.status == status).peekable();
    println!("{}", status.bold().underline().yellow());
    if tasks.peek().is_some() {
        for task in tasks {
            println!("{task}");
        }
    } else {
        println!("{}", "No Tasks".bright_red());
    }
}
