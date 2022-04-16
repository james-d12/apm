use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_dnf() -> PackageManager{
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install, true),
        Command::new("remove", CommandType::Uninstall, true),
        Command::new("reinstall", CommandType::Reinstall, true),
        Command::new("upgrade", CommandType::Update, false),
        Command::new("upgrade", CommandType::Upgrade, false),
        Command::new("search", CommandType::Search, true),
        Command::new("list", CommandType::List, false),
        Command::new("clean", CommandType::Clean, false),
    ];
    return PackageManager::new("Dandifed Yum Package Manager", "dnf", commands);
}