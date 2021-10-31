use std::process::Command;

enum Interpreter {
    Shell,
    CommandLine
}

fn execute_helper(interpreter: Interpreter, command: &str) -> bool {
    let output: std::process::ExitStatus;
    match interpreter {
        Interpreter::CommandLine => {
            output = Command::new("cmd").args(["/C", command]).status().unwrap();
        }
        Interpreter::Shell => {
            output = Command::new("sh").arg("-c").arg(command).status().unwrap();
        }
    }
    return output.success();
}

#[cfg(target_os = "windows")]
pub fn execute(command: &str) -> bool {
    return execute_helper(Interpreter::CommandLine, command);
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn execute(command: &str) -> bool {
    return execute_helper(Interpreter::Shell, command);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
  
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