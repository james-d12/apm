use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_choco() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install),
        Command::new("uninstall", CommandType::Uninstall),
        Command::new(
            "install --force --force-dependencies",
            CommandType::Reinstall,
        ),
        Command::new("upgrade all", CommandType::Update),
        Command::new("upgrade all", CommandType::Upgrade),
        Command::new("search", CommandType::Search),
        Command::new("list --local-only", CommandType::List),
        Command::new("help", CommandType::Help),
    ];
    return PackageManager::new("Chocolatey Package Manager", "choco", commands);
}
