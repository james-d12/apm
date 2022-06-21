use crate::apm::command::Command;
use crate::apm::command::CommandType;
use crate::apm::terminal;

pub trait PackageManagement {
    fn print(&self);
    fn execute(&self, command_type: CommandType, argument: &String) -> bool;
}

pub struct PackageManager {
    pub name: String,
    pub package_name: String,
    pub commands: Vec<Command>,
}

impl PackageManager {
    pub fn new(name: &str, package_name: &str, commands: Vec<Command>) -> PackageManager {
        PackageManager {
            name: name.to_string(),
            package_name: package_name.to_string(),
            commands,
        }
    }

    fn find_command(&self, command_type: CommandType) -> Option<&Command> {
        for command in self.commands.iter() {
            if command.command_type == command_type {
                return Some(command);
            }
        }
        return None;
    }
}

impl PackageManagement for PackageManager {
    fn print(&self) {
        println!("-------- {0} --------", self.name);
        println!("Package Manager Name: {0}", self.package_name);
        for command in self.commands.iter() {
            println!("{}", command);
        }
        println!("--------{:-<1$}--------", "", self.name.chars().count() + 2);
    }

    fn execute(&self, command_type: CommandType, argument: &String) -> bool {
        let command: Option<&Command> = self.find_command(command_type);

        match command {
            Some(x) => {
                let argument: String =
                    format!("{0} {1} {2}", self.package_name, x.name, argument).to_owned();
                let res = terminal::execute(&argument);
                return res;
            }
            None => {
                println!(
                    "Command type does not exist on current package manager: {}.",
                    self.name
                );
                return false;
            }
        }
    }
}
