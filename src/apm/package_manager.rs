#[path = "./command.rs"]
pub mod command;

use command::Command;
use command::CommandType;

mod StdCommand { use std::process::Command; }

pub trait PackageManagement {
    fn print(&self);
    fn execute(&self, command_type: CommandType, package_name: String) -> bool;
}

pub struct PackageManager {
    pub name: String,
    pub package_name: String,
    pub commands: Vec<Command>
}

impl PackageManager {
    pub fn new(name: &str, package_name: &str, commands: Vec<Command>) -> PackageManager { 
        PackageManager{
            name: name.to_string(),
            package_name: package_name.to_string(),
            commands: commands
        }
    }
    
    fn find_command(&self, command_type: CommandType) -> Option<&Command> {
        let iter = self.commands.iter();
        for command in iter {
            if command.command_type == command_type {
                return Some(command);
            }
        }
        return None;
    }
}

impl PackageManagement for PackageManager {
    fn print(&self) {
        println!("Name: {0}, Package Name: {1}", self.name, self.package_name);

        for command in self.commands.iter() {
            println!("{}", command);
        }
    }

    fn execute(&self, command_type: CommandType, package_name: String) -> bool {
        let command: Option<&Command> = self.find_command(command_type);

        match command {
            Some(x) => {
                if x.requires_package == true && package_name.is_empty() {
                    println!("Command: {} requires package.", x.name);
                }       
                
                return true;
            },
            None => { 
                println!("Found nothing.");
                return false
            },
        }

    }
}