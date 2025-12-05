use anyhow::{Result, anyhow};
use model::{
    Task,
    TaskList,
    parse_to_unix,
};
use view::get_confirmation;

// TODO
// - move out of lib.rs
// - move more to view

pub fn delete_task(task_vec: &mut TaskList, id: u16, noconfirm: bool) -> Result<()> {
    let task = task_vec.get_task_by_id(id)?;
    if noconfirm || get_confirmation(&task.name)? {
        println!("Deleting task '{}'", task.name);
        task_vec.task.retain(|task| task.id != id);
        for (i, task) in task_vec.task.iter_mut().enumerate() {
            task.id = i as u16 + 1;
        }
    }

    Ok(())
}

pub fn add_task(task_vec: &mut TaskList, name: &str, expiration: &str) -> Result<()> {
    task_vec.task.push(Task {
        id: task_vec.task.len() as u16 + 1,
        name: name.to_string(),
        finished: false,
        expiration: parse_to_unix(expiration)
    });
    println!("Adding task '{}'", name);

    Ok(())
}

pub fn edit_task(task_vec: &mut TaskList, id: u16, name: Option<&str>, expiration: Option<&str>) -> Result<()> {
    let task = task_vec.get_task_by_id(id)?;
    
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

pub fn finish_task(task_vec: &mut TaskList, id: u16) -> Result<()> {
    let task = task_vec.get_task_by_id(id)?;
    task.finished = !task.finished;
    if task.finished {
        println!("Finished task {}", id);
    } else {
        println!("Unfinished task {}", id);
    }

    Ok(())
}
