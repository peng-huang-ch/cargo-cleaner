use std::{path, process::Command};

/// Run the `cargo clean` command
pub fn clean(absolute_path: &path::Path) -> eyre::Result<()> {
    let command = "cargo";
    let args = ["clean"];
    let mut command = Command::new(command);
    let mut command = command.args(&args);
    execute_in_dir(absolute_path, &mut command)
}

/// Execute a command in a directory
fn execute_in_dir(absolute_path: &path::Path, command: &mut Command) -> eyre::Result<()> {
    let dir: path::PathBuf = absolute_path
        .canonicalize()
        .expect("Failed resolving an absolute path");
    let output = command
        .current_dir(dir.to_path_buf())
        .output()
        .expect("Failed to execute command");

    println!(
        "Running `{}` {:?} in directory: {:?}",
        command
            .get_program()
            .to_str()
            .unwrap_or("Invalid program name"),
        command.get_args().collect::<Vec<_>>(),
        dir.to_path_buf()
    );
    println!(
        "Status: {}",
        if output.status.success() {
            "😊"
        } else {
            "😭"
        }
    );
    println!(
        "Output:\n{}",
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_cmd() {
        let current_dir = env::current_dir().expect("Failed to get current dir");
        println!(
            "Entries modified in the last 24 hours in {:?}:",
            current_dir
        );
        // let mut command = Command::new("ls");
        // clean(current_dir.as_path(), &mut command).expect("Failed to list all file");
        let mut cmd = Command::new("ls");
        let mut command = cmd.args(&["-f"]);
        execute_in_dir(current_dir.as_path(), &mut command).expect("Failed to list all file");
    }
}
