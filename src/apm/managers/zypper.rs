use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_zypper() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install),
        Command::new("rm", CommandType::Uninstall),
        Command::new("install --force", CommandType::Reinstall),
        Command::new("refresh", CommandType::Update),
        Command::new("update", CommandType::Upgrade),
        Command::new("search", CommandType::Search),
        Command::new("", CommandType::List),
        Command::new("", CommandType::Clean),
    ];
    return PackageManager::new("Zypper Package Manager", "zypper", commands);
}
