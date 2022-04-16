use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_pacman() -> PackageManager{
    let commands: Vec<Command> = vec![
        Command::new("-S", CommandType::Install, true),
        Command::new("-R", CommandType::Uninstall, true),
        Command::new("-S", CommandType::Reinstall, true),
        Command::new("-Syy", CommandType::Update, false),
        Command::new("-Syu", CommandType::Upgrade, false),
        Command::new("-Ss", CommandType::Search, true),
        Command::new("-Q", CommandType::List, false),
        Command::new("-Sc", CommandType::Clean, false),
    ];
    return PackageManager::new("Pacman Package Manager", "pacman", commands);
}