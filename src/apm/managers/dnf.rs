use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_dnf() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install),
        Command::new("remove", CommandType::Uninstall),
        Command::new("reinstall", CommandType::Reinstall),
        Command::new("upgrade", CommandType::Update),
        Command::new("upgrade", CommandType::Upgrade),
        Command::new("search", CommandType::Search),
        Command::new("list", CommandType::List),
        Command::new("clean", CommandType::Clean),
    ];
    return PackageManager::new("Dandifed Yum Package Manager", "dnf", commands);
}
