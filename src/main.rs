mod apm;

use apm::arguments;
use apm::checker;
use apm::command::Command;
use apm::command::CommandType;
use apm::package_manager::PackageManager;
use apm::package_manager::PackageManagement;
 
fn match_command(package_manager: PackageManager, args: arguments::Argument) {
    match args.command_argument.as_str() {
        "install" => {  package_manager.execute(CommandType::Install, args.package_argument); },
        "reinstall" => { package_manager.execute(CommandType::Reinstall, args.package_argument); },
        "uninstall" => { package_manager.execute(CommandType::Uninstall, args.package_argument); },
        "update" => {  package_manager.execute(CommandType::Update, args.package_argument); },
        "upgrade" => {  package_manager.execute(CommandType::Upgrade, args.package_argument); },
        "search" => {  package_manager.execute(CommandType::Search, args.package_argument); },
        "list" => { package_manager.execute(CommandType::List, args.package_argument); },
        "clean" => {  package_manager.execute(CommandType::Clean, args.package_argument); },
        "help" => { package_manager.execute(CommandType::Help, args.package_argument); },
        _ => {  println!("Command: {} is invalid.", args.package_argument) },
    }
}

fn main() {
    let package_manager: Option<PackageManager> = apm::managers::decide_package_manager();
    
    match package_manager {
        Some(package_manager) => { 
            let args: arguments::Argument = arguments::get_arguments();

            println!("{0} {1} {2}", args.command_argument, args.package_argument, args.switch_argument);
            
            if args.command_argument == "--help" {
                println!("Help");
                std::process::exit(0);
            } else if args.command_argument == "--info" {
                package_manager.print();
                std::process::exit(0);
            } else if args.command_argument == "--local" {
                if checker::check_is_npm() {
                    println!("Ok, running with npm");
                } 
                std::process::exit(0);
            }

            match_command(package_manager, args);
        },
        None => { 
            println!("Could not find appropriate package manager for current system.");
            std::process::exit(1);
        },
    }
}
 