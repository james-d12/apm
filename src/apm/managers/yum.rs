use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_yum_manager() -> PackageManager{
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install, true),
        Command::new("remove", CommandType::Uninstall, true),
        Command::new("reinstall", CommandType::Reinstall, true),
        Command::new("update", CommandType::Update, false),
        Command::new("update", CommandType::Upgrade, false),
        Command::new("search", CommandType::Search, true),
        Command::new("list", CommandType::List, false),
        Command::new("clean all", CommandType::Clean, false),
    ];
    return PackageManager::new("Yum Package Manager", "yum", commands);
}