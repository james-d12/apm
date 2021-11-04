#[derive(PartialEq, PartialOrd)]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum CommandType {
    Install, 
    Reinstall, 
    Uninstall, 
    Update, 
    Upgrade,
    Search, 
    List,
    Clean,
    Help,
    Unknown
}

pub fn get_command_type_as_string(command_type: CommandType) -> String {
    return command_type.to_string().to_lowercase();
}

#[derive(PartialEq, PartialOrd)]
pub struct Command {
    pub name: String,
    pub command_type: CommandType,
    pub requires_package: bool,
}

impl Command {
    pub fn new(name: &str, command_type: CommandType, requires_package: bool) -> Command {
        Command{
            name: name.to_string(),
            command_type: command_type,
            requires_package: requires_package
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let command_type = get_command_type_as_string(self.command_type);
        write!(f, "[Name: {0} | Command Type: {1} | Requires Package: {2}]", self.name, command_type, self.requires_package)
    }
}

impl std::fmt::Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::CommandType;

    #[test]
    fn test_constructor() {
        let command_name: &str = "install";
        let command_type: CommandType = CommandType::Install;
        let requires_package: bool = true;

        let command: Command = Command::new(command_name, command_type, requires_package);

        assert_eq!(command.name, command_name);
        assert_eq!(command.command_type, command_type);
        assert_eq!(command.requires_package, requires_package);
    }

}
