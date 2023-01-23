use crate::apm::command::Command;
use crate::apm::command::CommandType;
use crate::apm::terminal;

pub trait PackageManagement {
    fn print(&self);
    fn execute(&self, command_type: CommandType, argument: &str, message: &str) -> bool;
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
        self.commands
            .iter()
            .find(|&command| command.command_type == command_type)
    }
}

impl PackageManagement for PackageManager {
    fn print(&self) {
        println!("------------- {0} -------------", self.name);
        for command in self.commands.iter() {
            println!("- {}", command);
        }
        println!(
            "-------------{:-<1$}-------------",
            "",
            self.name.chars().count() + 2
        );
    }

    fn execute(&self, command_type: CommandType, argument: &str, message: &str) -> bool {
        let command: Option<&Command> = self.find_command(command_type);

        match command {
            Some(x) => {
                println!("{} {}", message, argument);
                let argument = format!("{0} {1} {2}", self.package_name, x.name, argument);
                terminal::execute(&argument)
            }
            None => {
                println!(
                    "Command type does not exist on current package manager: {}.",
                    self.name
                );
                false
            }
        }
    }
}
