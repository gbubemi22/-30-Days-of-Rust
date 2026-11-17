use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let source = "days/20-file-handling/examples/notes.txt";
    let contents = fs::read_to_string(source)?;
    let summary = format!("{} lines read", contents.lines().count());

    let mut output = PathBuf::from(std::env::temp_dir());
    output.push("rust-course-file-summary.txt");
    fs::write(&output, summary)?;

    let saved = fs::read_to_string(&output)?;
    println!("{saved}");
    Ok(())
}
