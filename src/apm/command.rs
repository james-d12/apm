#[derive(PartialEq, PartialOrd)]
pub enum CommandType {
    Install, 
    Reinstall, 
    Uninstall, 
    Update, 
    Upgrade,
    Search, 
    List,
    Clean
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
        let mut command_type: &str = "";
        match self.command_type{
            CommandType::Install => command_type = "install",
            CommandType::Reinstall => command_type = "reinstall",
            CommandType::Uninstall => command_type = "uninstall",
            CommandType::Update => command_type = "update",
            CommandType::Upgrade => command_type = "upgrade",
            CommandType::Search => command_type = "search",
            CommandType::List => command_type = "list",
            CommandType::Clean => command_type = "clean",
        }

        write!(f, "(Name a: {0}, Command Type: {1}, Requires Package: {2})", 
            self.name, command_type, self.requires_package)
    }
}


