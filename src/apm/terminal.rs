use std::process::Command;

#[cfg(target_os = "windows")]
pub fn execute(command: &str) -> bool {
    return Command::new("cmd")
        .args(["/C", command])
        .status()
        .unwrap()
        .success();
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn execute(command: &str) -> bool {
    return Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .unwrap()
        .success();
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_os = "windows")]
    fn test_execute() {
        assert_eq!(super::execute("dir"), true);
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_execute() {
        assert_eq!(super::execute("ls"), true);
    }
}
