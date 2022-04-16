use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_choco() -> PackageManager{
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install, true),
        Command::new("uninstall", CommandType::Uninstall, true),
        Command::new("install --force --force-dependencies", CommandType::Reinstall, true),
        Command::new("upgrade all", CommandType::Update, false),
        Command::new("upgrade all", CommandType::Upgrade, false),
        Command::new("search", CommandType::Search, true),
        Command::new("list --local-only", CommandType::List, false),
        Command::new("help", CommandType::Help, false),
    ];
    return PackageManager::new("Chocolatey Package Manager", "choco", commands);
}