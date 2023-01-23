use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn npm() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install),
        Command::new("uninstall", CommandType::Uninstall),
        Command::new("install", CommandType::Reinstall),
        Command::new("update", CommandType::Update),
        Command::new("update", CommandType::Upgrade),
    ];
    PackageManager::new("Node Package Manager", "npm", commands)
}
