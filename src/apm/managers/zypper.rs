use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_zypper() -> PackageManager{
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install, true),
        Command::new("rm", CommandType::Uninstall, true),
        Command::new("install --force", CommandType::Reinstall, true),
        Command::new("refresh", CommandType::Update, false),
        Command::new("update", CommandType::Upgrade, false),
        Command::new("search", CommandType::Search, true),
        Command::new("", CommandType::List, false),
        Command::new("", CommandType::Clean, false),
    ];
    return PackageManager::new("Zypper Package Manager", "zypper", commands);
}