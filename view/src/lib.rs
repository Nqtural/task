use anyhow::Result;
use chrono::Utc;
use colored::*;
use std::io::{self, Write};
use model::{
    TaskList,
    unix_to_relative,
};

// TODO
// - move functions out of lib.rs

pub fn get_confirmation(task_name: &str) -> Result<bool> {
    print!("Are you sure you want to delete task '{}'? [y/N]: ", task_name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: failed to read line");
    
    Ok(input.to_lowercase().contains("y"))
}

pub fn print_tasks(task_vec: &TaskList) -> Result<()> {
    if task_vec.task.is_empty() {
        println!("No tasks yet. Create one with `task add \"My task\"`");
    }

    let id_width = task_vec.task
        .iter()
        .map(|task| task.id.to_string().len())
        .max()
        .unwrap_or(1);
    let name_width = task_vec.task
        .iter()
        .map(|task| task.name.len())
        .max()
        .unwrap_or(10);

    for task in &task_vec.task {
        let last_column = if task.finished {
            "DONE".to_string().green()
        } else if let Some(exp) = task.expiration {
            if exp - Utc::now().timestamp() <= 0 {
                unix_to_relative(exp).red()
            } else {
                unix_to_relative(exp).bright_black()
            }
        } else {
            "".to_string().white()
        };
        print!("{: >id_width$}. ", task.id, id_width = id_width + 1);
        print!("{: <name_width$} ", task.name.cyan().bold(), name_width = name_width);
        println!("{: >15}", last_column);
    }

    Ok(())
}
