use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn yum() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install),
        Command::new("remove", CommandType::Uninstall),
        Command::new("reinstall", CommandType::Reinstall),
        Command::new("update", CommandType::Update),
        Command::new("update", CommandType::Upgrade),
        Command::new("search", CommandType::Search),
        Command::new("list", CommandType::List),
        Command::new("clean all", CommandType::Clean),
    ];
    return PackageManager::new("Yum Package Manager", "yum", commands);
}
