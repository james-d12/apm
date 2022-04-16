use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_eopkg() -> PackageManager{
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install, true),
        Command::new("remove", CommandType::Uninstall, true),
        Command::new("install --reinstall", CommandType::Reinstall, true),
        Command::new("upgrade", CommandType::Update, false),
        Command::new("upgrade", CommandType::Upgrade, false),
        Command::new("search", CommandType::Search, true),
        Command::new("list", CommandType::List, false),
        Command::new("li -l", CommandType::Clean, false),
    ];
    return PackageManager::new("EOPKG Package Manager", "eopkg", commands);
}