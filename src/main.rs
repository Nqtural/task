use clap::{Parser, Subcommand};
use chrono::{NaiveDate, NaiveDateTime, Utc, Duration, Datelike, Local, TimeZone};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{io::{self, Write}, fs};
use colored::*;
use anyhow::{Result, anyhow};

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    id: u16,
    name: String,
    finished: bool,
    expiration: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TaskList {
    task: Vec<Task>,
}

fn read_tasks() -> Result<TaskList> {
    match fs::read_to_string("tasks.toml") {
        Ok(toml_str) => {
            let task_list: TaskList = toml::from_str(&toml_str)?;
            Ok(task_list)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // File doesn't exist – create an empty TaskList and write it
            let task_list = TaskList { task: Vec::new() };
            let toml_str = toml::to_string_pretty(&task_list)?;
            fs::write("tasks.toml", toml_str)?;
            Ok(task_list)
        }
        Err(e) => Err(e.into()),
    }
}

fn get_confirmation(task_name: &str) -> Result<bool> {
    print!("Are you sure you want to delete task '{}'? [y/N]: ", task_name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: failed to read line");
    
    Ok(input.to_lowercase().contains('y'))
}

fn get_task_by_id(task_vec: &mut TaskList, id: u16) -> Result<&mut Task> {
    task_vec.task.iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| anyhow!("Task with id {} not found", id))
}

fn print_tasks(task_vec: &TaskList) -> Result<()> {
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

fn delete_task(task_vec: &mut TaskList, id: u16, noconfirm: bool) -> Result<()> {
    let task = get_task_by_id(task_vec, id)?;
    if noconfirm || get_confirmation(&task.name)? {
        println!("Deleting task '{}'", task.name);
        task_vec.task.retain(|task| task.id != id);
        for (i, task) in task_vec.task.iter_mut().enumerate() {
            task.id = i as u16 + 1;
        }
    }

    Ok(())
}

fn add_task(task_vec: &mut TaskList, name: &str, expiration: &str) -> Result<()> {
    task_vec.task.push(Task {
        id: task_vec.task.len() as u16 + 1,
        name: name.to_string(),
        finished: false,
        expiration: parse_to_unix(expiration)
    });
    println!("Adding task '{}'", name);

    Ok(())
}

fn edit_task(task_vec: &mut TaskList, id: u16, name: Option<&str>, expiration: Option<&str>) -> Result<()> {
    let task = get_task_by_id(task_vec, id)?;
    
    if let Some(n) = name {
        println!("Changing '{}' -> '{}'", task.name, n);
        task.name = n.to_string();
    }
    
    if let Some(exp) = expiration {
        let ts = parse_to_unix(exp)
            .ok_or_else(|| anyhow!("Invalid expiration date: {}", expiration.unwrap_or("<none>")));
        task.expiration = Some(ts?);
        println!("Updated expiration for '{}'", task.name);
    }

    Ok(())
}

fn finish_task(task_vec: &mut TaskList, id: u16) -> Result<()> {
    let task = get_task_by_id(task_vec, id)?;
    task.finished = !task.finished;
    if task.finished {
        println!("Finished task {}", id);
    } else {
        println!("Unfinished task {}", id);
    }

    Ok(())
}

fn parse_to_unix(input: &str) -> Option<i64> {
    let now = Local::now();
    
    // relative durations
    let re_relative = Regex::new(r"(?i)(?:(\d+)y)?(?:(\d+)m)?(?:(\d+)w)?(?:(\d+)d)?(?:(\d+)h)?(?:(\d+)min)?$").unwrap();
    if let Some(caps) = re_relative.captures(input) {
        let mut duration = Duration::seconds(0);
        if let Some(y) = caps.get(1) { duration += Duration::days(y.as_str().parse::<i64>().unwrap() * 365); }
        if let Some(m) = caps.get(2) { duration += Duration::days(m.as_str().parse::<i64>().unwrap() * 30); }
        if let Some(w) = caps.get(3) { duration += Duration::days(w.as_str().parse::<i64>().unwrap() * 7); }
        if let Some(d) = caps.get(4) { duration += Duration::days(d.as_str().parse::<i64>().unwrap()); }
        if let Some(h) = caps.get(5) { duration += Duration::hours(h.as_str().parse::<i64>().unwrap()); }
        if let Some(min) = caps.get(6) { duration += Duration::minutes(min.as_str().parse::<i64>().unwrap()); }
        if duration != Duration::seconds(0) {
            return Some((now + duration).timestamp());
        }
    }

    // time only: HH:MM (today, local)
    let re_time = Regex::new(r"^(\d{2}):(\d{2})$").unwrap();
    if let Some(caps) = re_time.captures(input) {
        let hour = caps.get(1).unwrap().as_str().parse::<u32>().ok()?;
        let minute = caps.get(2).unwrap().as_str().parse::<u32>().ok()?;
        let date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())?;
        let datetime = NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(hour, minute, 0)?);
        let local_dt = Local.from_local_datetime(&datetime).single()?;
        return Some(local_dt.timestamp());
    }

    // absolute date/time format: DDMM[YY][-HH:MM], local
    let re_abs = Regex::new(r"^(\d{2})(\d{2})(\d{2})?(?:-(\d{2}):(\d{2}))?$").unwrap();
    if let Some(caps) = re_abs.captures(input) {
        let day = caps.get(1).unwrap().as_str().parse::<u32>().ok()?;
        let month = caps.get(2).unwrap().as_str().parse::<u32>().ok()?;
        let year = if let Some(y) = caps.get(3) {
            2000 + y.as_str().parse::<i32>().ok()?  // assuming 2000+
        } else {
            now.year()
        };
        let hour = caps.get(4).map_or(0, |h| h.as_str().parse::<u32>().unwrap());
        let minute = caps.get(5).map_or(0, |m| m.as_str().parse::<u32>().unwrap());

        let date = NaiveDate::from_ymd_opt(year, month, day)?;
        let datetime = NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(hour, minute, 0)?);
        let local_dt = Local.from_local_datetime(&datetime).single()?;
        return Some(local_dt.timestamp());
    }

    None
}

fn unix_to_relative(unix_time: i64) -> String {
    let now = Utc::now().timestamp();
    let mut seconds = unix_time - now;
    let negative = seconds < 0;
    if negative {
        seconds = -seconds;
    }

    let units = [
        ("y", 365 * 24 * 3600),
        ("mo", 30 * 24 * 3600),
        ("w", 7 * 24 * 3600),
        ("d", 24 * 3600),
        ("h", 3600),
        ("m", 60),
        ("s", 1),
    ];

    let mut values: Vec<(i64, &str)> = units
        .iter()
        .map(|&(name, unit_sec)| {
            let val = seconds / unit_sec;
            seconds %= unit_sec;
            (val, name)
        })
        .collect();

    let overflow_map = |unit: &str| match unit {
        "s" => 60,
        "m" => 60,
        "h" => 24,
        "d" => 7,
        "w" => 4,
        "mo" => 12,
        _ => 0
    };

    loop {
        let mut changed = false;
        for i in (1..values.len()).rev() {
            let (val, unit) = values[i];
            let overflow = overflow_map(unit);
            if overflow > 0 && val >= overflow {
                let carry = val / overflow;
                values[i].0 %= overflow;
                values[i - 1].0 += carry;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let result: Vec<String> = values
        .iter()
        .filter(|&&(v, _)| v > 0)
        .take(2)
        .map(|&(v, u)| format!("{}{}", v, u))
        .collect();

    let output = if result.is_empty() { "0s".to_string() } else { result.join(" ") };

    if negative {
        format!("Overdue {}", output)
    } else {
        output
    }
}

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Simple CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List tasks
    List,
    /// Add a new task
    Add {
        name: String,
        #[arg(short, long)]
        time: Option<String>,
    },
    /// Delete a task by ID (use --no-confirm to skip prompt)
    Delete {
        id: u16,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        no_confirm: bool,
    },
    /// Edit a task (use --name or -n for name and --time or -t for expire time)
    Edit {
        id: u16,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        time: Option<String>,
    },
    /// Toggle finished status
    Finish {
        id: u16,
    },
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let mut task_vec = read_tasks()?;

    match cli.command {
        Commands::List => {
            print_tasks(&task_vec)?;
        }
        Commands::Add { name, time } => {
            add_task(&mut task_vec, &name, time.as_deref().unwrap_or(""))?;
        }
        Commands::Delete { id, no_confirm } => {
            delete_task(&mut task_vec, id, no_confirm)?;
        }
        Commands::Edit { id, name, time } => {
            if name.is_none() && time.is_none() {
                return Err(anyhow!("--edit requires either '--name <NAME>' or '--time <TIME>'"));
            }
            edit_task(&mut task_vec, id, name.as_deref(), time.as_deref())?;
        }
        Commands::Finish { id } => {
            finish_task(&mut task_vec, id)?;
        }
    }

    let toml_str = toml::to_string_pretty(&task_vec)?;
    fs::write("tasks.toml", toml_str)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
