use clap::{arg, ArgMatches, Command};

use crate::CommandType;
use crate::PackageManagement;
use crate::PackageManager;

pub fn get_cli() -> Command<'static> {
    Command::new("Agnostic Package Manager")
        .version("0.2")
        .author("James Durban")
        .about("Manages the installation and usage of package managers across various platforms.")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .short_flag('i')
                .long_flag("install")
                .about("installs a provided package.")
                .arg(arg!(<PACKAGE> "The package to install"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("uninstall")
                .short_flag('r')
                .long_flag("uninstall")
                .about("Uninstalls a provided package.")
                .arg(arg!(<PACKAGE> "The package to uninstall"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("reinstall")
                .short_flag('e')
                .long_flag("reinstall")
                .about("Reinstalls a provided package.")
                .arg(arg!(<PACKAGE> "The package to reinstall"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("update")
                .short_flag('u')
                .long_flag("update")
                .about("Updates repositories."),
        )
        .subcommand(
            Command::new("upgrade")
                .short_flag('g')
                .long_flag("upgrade")
                .about("Upgrades a package to latest version.")
                .arg(arg!(<PACKAGE> "The package to upgrade."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("search")
                .short_flag('s')
                .long_flag("search")
                .about("Searches for a package.")
                .arg(arg!(<PACKAGE> "The package to search for"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("list")
                .short_flag('l')
                .long_flag("list")
                .about("Lists all installed packages."),
        )
        .subcommand(
            Command::new("outdated")
                .short_flag('o')
                .long_flag("outdated")
                .about("Lists all outdated packages."),
        )
        .subcommand(
            Command::new("clean")
                .short_flag('c')
                .long_flag("clean")
                .about("Cleans package manager's cache"),
        )
        .subcommand(
            Command::new("info")
                .short_flag('f')
                .long_flag("info")
                .about("Show's info about apm."),
        )
}

fn get_argument(sub_matches: &ArgMatches, name: &str) -> String {
    let argument = sub_matches.get_one::<String>(name);

    match argument {
        Some(name) => name.to_string(),
        _ => "".to_string(),
    }
}

pub fn match_command(matches: &ArgMatches, package_manager: &PackageManager) -> bool {
    let mut result = true;
    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Installing {}", argument);
            result = package_manager.execute(CommandType::Install, &argument);
        }
        Some(("uninstall", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Uninstalling {}", argument);
            result = package_manager.execute(CommandType::Uninstall, &argument);
        }
        Some(("reinstall", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Reinstalling {}", argument);
            result = package_manager.execute(CommandType::Reinstall, &argument);
        }
        Some(("update", _sub_matches)) => {
            print!("Updating repos");
            result = package_manager.execute(CommandType::Update, &"".to_string());
        }
        Some(("upgrade", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Upgrading {}", argument);
            result = package_manager.execute(CommandType::Upgrade, &argument);
        }
        Some(("search", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Searching for {}", argument);
            result = package_manager.execute(CommandType::Search, &argument);
        }
        Some(("list", _sub_matches)) => {
            print!("Listing all installed packages");
            result = package_manager.execute(CommandType::List, &"".to_string());
        }
        Some(("outdated", _sub_matches)) => {
            print!("Listing all outdated packages");
            result = package_manager.execute(CommandType::Outdated, &"".to_string());
        }
        Some(("clean", _sub_matches)) => {
            print!("Cleaning");
            result = package_manager.execute(CommandType::Clean, &"".to_string());
        }
        Some(("info", _sub_matches)) => package_manager.print(),
        _ => {
            println!("Invalid command.");
        }
    }
    return result;
}
