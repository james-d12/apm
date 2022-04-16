use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn get_apk() -> PackageManager{
    let commands: Vec<Command> = vec![
        Command::new("add", CommandType::Install, true),
        Command::new("del", CommandType::Uninstall, true),
        Command::new("reinstall", CommandType::Reinstall, true),
        Command::new("update", CommandType::Update, false),
        Command::new("upgrade", CommandType::Upgrade, false),
        Command::new("search", CommandType::Search, true),
        Command::new("info -vv | sort", CommandType::List, false),
        Command::new("cache clean", CommandType::Clean, false),
    ];
    return PackageManager::new("Alpine Package Manager", "apk", commands);
}