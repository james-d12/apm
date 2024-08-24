use crate::Command;
use crate::CommandType;
use crate::PackageManager;

pub fn nuget() -> PackageManager {
    let commands: Vec<Command> = vec![
        Command::new("add", CommandType::Install),
        Command::new("delete", CommandType::Uninstall),
        Command::new("add", CommandType::Reinstall),
        Command::new("update", CommandType::Update)
    ];
    PackageManager::new("Nuget Package Manager", "dotnet nuget", commands)
}
