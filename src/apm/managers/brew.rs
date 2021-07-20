use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_brew_manager() -> PackageManager{
    let commands: Vec<Command> = vec![
        Command::new("", CommandType::Install, true),
        Command::new("", CommandType::Uninstall, true),
        Command::new("", CommandType::Reinstall, true),
        Command::new("", CommandType::Update, false),
        Command::new("", CommandType::Upgrade, false),
        Command::new("", CommandType::Search, true),
        Command::new("", CommandType::List, false),
        Command::new("", CommandType::Clean, false),
    ];
    return PackageManager::new("Homebrew Package Manager", "brew", commands);
}