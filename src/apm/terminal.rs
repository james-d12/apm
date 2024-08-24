use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
pub fn execute(command: &str) -> bool {
    let result = Command::new("cmd").args(["/C", command]).status();

    match result {
        Ok(status) => status.success(),
        Err(error) => {
            println!("Error: {0} whilst trying to execute {1}.", error, command);
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn check_executable_exists(executable: &str) -> bool {
    let result = Command::new("cmd")
        .args(["/C", executable])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match result {
        Ok(status) => status.success(),
        Err(error) => {
            println!(
                "Error: {0} whilst trying to check executable {1} exists.",
                error, executable
            );
            false
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn execute(command: &str) -> bool {
    let result = Command::new("sh").arg("-c").arg(command).status();

    match result {
        Ok(status) => status.success(),
        Err(error) => {
            println!(
                "Error: {0} whilst trying to execute {1}.",
                error.to_string(),
                command
            );
            false
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn check_executable_exists(executable: &str) -> bool {
    let result = Command::new("sh")
        .arg("-c")
        .arg(executable)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match result {
        Ok(status) => status.success(),
        Err(error) => {
            println!(
                "Error: {0} whilst trying to check executable {1} exists.",
                error.to_string(),
                executable
            );
            false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_os = "windows")]
    fn test_execute() {
        assert_eq!(super::execute("dir"), true);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_check_executable_exists() {
        assert_eq!(super::check_executable_exists("cd"), true);
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_execute() {
        assert_eq!(super::execute("ls"), true);
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_check_executable_exists() {
        assert_eq!(super::check_executable_exists("cd"), true);
    }
}
