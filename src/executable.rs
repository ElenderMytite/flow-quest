use std::fs;
use std::io::{self, Write};
use std::process::Command;
pub fn create_exe_file(code: String) -> io::Result<()> {
    // Create a temporary file for the assembly code
    let temp_file_path = "temp.s";
    let mut temp_file = fs::File::create(temp_file_path)?;
    temp_file.write_all(code.as_bytes())?;

    // Compile the assembly code using gcc
    let output = Command::new("gcc")
        .args(&["-nostartfiles", "-static", temp_file_path, "-o", "output"])
        .output()?;

    // Check if the compilation was successful
    if output.status.success() {
        println!("Compilation successful! Executable created: output");
    } else {
        eprintln!("Compilation failed:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    fs::remove_file(temp_file_path)?;

    Ok(())
}
