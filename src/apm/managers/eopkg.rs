use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn eopkg() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("install", CommandType::Install),
        Command::new("remove", CommandType::Uninstall),
        Command::new("install --reinstall", CommandType::Reinstall),
        Command::new("upgrade", CommandType::Update),
        Command::new("upgrade", CommandType::Upgrade),
        Command::new("search", CommandType::Search),
        Command::new("list", CommandType::List),
        Command::new("li -l", CommandType::Clean),
    ];
    return PackageManager::new("EOPKG Package Manager", "eopkg", commands);
}
