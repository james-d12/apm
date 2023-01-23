use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn pacman() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("-S", CommandType::Install),
        Command::new("-R", CommandType::Uninstall),
        Command::new("-S", CommandType::Reinstall),
        Command::new("-Syy", CommandType::Update),
        Command::new("-Syu", CommandType::Upgrade),
        Command::new("-Ss", CommandType::Search),
        Command::new("-Q", CommandType::List),
        Command::new("-Sc", CommandType::Clean),
    ];
    PackageManager::new("Pacman Package Manager", "pacman", commands)
}
