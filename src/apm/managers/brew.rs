use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_brew() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("", CommandType::Install),
        Command::new("", CommandType::Uninstall),
        Command::new("", CommandType::Reinstall),
        Command::new("", CommandType::Update),
        Command::new("", CommandType::Upgrade),
        Command::new("", CommandType::Search),
        Command::new("", CommandType::List),
        Command::new("", CommandType::Clean),
    ];
    return PackageManager::new("Homebrew Package Manager", "brew", commands);
}
