#[path = "./package_manager.rs"]
mod package_manager;

use std::env;
use std::path::Path;

use package_manager::PackageManager;
use package_manager::PackageManagement;

const VALID_SWITCH_ARGUMENTS: [&str; 3] = [
    "--help", "--info", "--local"
];

const VALID_COMMAND_ARGUMENTS: [&str; 6] = [
    "install", "uninstall", "update", "upgrade",
    "search", "list"
];

fn get_command_argument(env_args: &std::vec::Vec<std::string::String>) -> &str {
    if env_args.len() < 2 {
        println!("No command provided.");
        std::process::exit(1);
    }
    let argument: &str = &env_args[1];
    return if VALID_COMMAND_ARGUMENTS.contains(&argument) { argument } else { "" }
}   

fn get_switch_argument(env_args: &std::vec::Vec<std::string::String>) -> &str {
    if env_args.len() < 2 {
        println!("No switch provided.");
        return "";
    }
    let argument: &str = &env_args[3];
    return if VALID_SWITCH_ARGUMENTS.contains(&argument) { argument } else { "" }
}

pub struct Argument {
    pub command_argument: String,
    pub package_argument: String,
    pub switch_argument: String
}

pub fn get_arguments() -> Argument {
    let env_args = env::args().collect::<Vec<String>>();
    let length = env_args.len() - 1;
    let (bin, _remainer) = env_args.split_first().unwrap();
    let _bin = Path::new(bin).file_stem().unwrap().to_str().unwrap();
    
    // Get and Set the command argument.
    let command_argument: &str = get_command_argument(&env_args);
    // Get and Set the package argument.
    let package_argument: &str = if length >= 2 { &env_args[2] } else { "" };
    // Get and Set the switch argument.
    let switch_argument: &str = if length >= 3 { get_switch_argument(&env_args) } else { "" };

    return Argument {
        command_argument: command_argument.to_string(),
        package_argument: package_argument.to_string(),
        switch_argument: switch_argument.to_string()
    };
}

pub fn process_arguments(package_manager: &PackageManager) {
    let arguments: Argument = get_arguments();

    if arguments.command_argument == "--help" {
        println!("Help");
        std::process::exit(0);
    } else if arguments.command_argument == "--info" {
        package_manager.print();
        std::process::exit(0);
    } else if arguments.command_argument == "--local" {
        std::process::exit(0);
    }

}