use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub description: String,
    pub complete: bool,
}

pub fn load_tasks(path: &Path) -> io::Result<Vec<Task>> {
    let json = match fs::read_to_string(path) {
        Ok(json) => json,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };

    serde_json::from_str(&json).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub fn save_tasks(path: &Path, tasks: &[Task]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write(path, json)
}

pub fn run(args: &[String], path: &Path, output: &mut impl Write) -> io::Result<()> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: task-manager add TEXT | list | done NUMBER",
        ));
    };

    let mut tasks = load_tasks(path)?;
    match command {
        "add" => {
            let description = args[1..].join(" ");
            if description.trim().is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "task description cannot be empty",
                ));
            }
            tasks.push(Task {
                description,
                complete: false,
            });
            save_tasks(path, &tasks)?;
            writeln!(output, "Added task {}", tasks.len())?;
        }
        "list" => {
            if tasks.is_empty() {
                writeln!(output, "No tasks yet.")?;
            }
            for (index, task) in tasks.iter().enumerate() {
                let marker = if task.complete { 'x' } else { ' ' };
                writeln!(output, "{}. [{}] {}", index + 1, marker, task.description)?;
            }
        }
        "done" => {
            let number = args
                .get(1)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "usage: task-manager done NUMBER",
                    )
                })?
                .parse::<usize>()
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
            if number == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "task numbers start at 1",
                ));
            }
            let task = tasks.get_mut(number - 1).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("no task numbered {number}"),
                )
            })?;
            task.complete = true;
            save_tasks(path, &tasks)?;
            writeln!(output, "Completed task {number}")?;
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unknown command; use add, list, or done",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{load_tasks, run, Task};
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after the Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("task-manager-{}-{nonce}.json", std::process::id()))
    }

    #[test]
    fn missing_file_starts_with_no_tasks() {
        let path = temp_path();
        assert!(load_tasks(&path).unwrap().is_empty());
    }

    #[test]
    fn add_list_and_complete_task() {
        let path = temp_path();
        let mut output = Vec::new();

        run(&["add".into(), "Read Rust".into()], &path, &mut output).unwrap();
        run(&["list".into()], &path, &mut output).unwrap();
        run(&["done".into(), "1".into()], &path, &mut output).unwrap();

        let tasks = load_tasks(&path).unwrap();
        assert_eq!(
            tasks,
            vec![Task {
                description: String::from("Read Rust"),
                complete: true,
            }]
        );
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn malformed_data_is_an_invalid_data_error() {
        let path = temp_path();
        fs::write(&path, "not json").unwrap();
        let error = load_tasks(&path).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn task_number_zero_is_rejected() {
        let path = temp_path();
        let error = run(&["done".into(), "0".into()], &path, &mut Vec::new()).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    }
}
