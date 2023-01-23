use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn apk() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("add", CommandType::Install),
        Command::new("del", CommandType::Uninstall),
        Command::new("reinstall", CommandType::Reinstall),
        Command::new("update", CommandType::Update),
        Command::new("upgrade", CommandType::Upgrade),
        Command::new("search", CommandType::Search),
        Command::new("info -vv | sort", CommandType::List),
        Command::new("cache clean", CommandType::Clean),
    ];
    PackageManager::new("Alpine Package Manager", "apk", commands)
}
