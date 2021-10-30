use std::process::Command;

#[cfg(target_os = "windows")]
pub fn execute(command: &str) -> bool {
    let output = Command::new("cmd")
        .args(["/C", command])
        .spawn();
    return true;
}

#[cfg(target_os = "linux")]
pub fn execute(command: &str) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    return true;
}

#[cfg(target_os = "macos")]
pub fn execute(command: &str) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    return true;
}
