use cli_task_manager::run;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = std::env::var_os("TASKS_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("tasks.json"));
    let stdout = std::io::stdout();
    let mut output = stdout.lock();

    match run(&args, &path, &mut output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
