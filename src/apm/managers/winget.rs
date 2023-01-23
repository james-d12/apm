use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn winget() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install),
        Command::new("uninstall", CommandType::Uninstall),
        Command::new("install", CommandType::Reinstall),
        Command::new("upgrade --include-unknown", CommandType::Update),
        Command::new("upgrade", CommandType::Upgrade),
        Command::new("search", CommandType::Search),
        Command::new("list", CommandType::List),
        Command::new("help", CommandType::Help),
    ];
    PackageManager::new("Winget Package Manager", "winget", commands)
}
