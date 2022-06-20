use clap::{arg, ArgMatches, Command};

use crate::CommandType;
use crate::PackageManagement;
use crate::PackageManager;

pub fn get_cli() -> Command<'static> {
    Command::new("Agnostic Package Manager")
        .version("0.2")
        .author("James Durban")
        .about("Manages the installation and usage of package managers across various platforms..")
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
            Command::new("clean")
                .short_flag('c')
                .long_flag("clean")
                .about("Cleans package manager's cache"),
        )
}

fn get_argument(sub_matches: &ArgMatches, name: &str) -> String {
    let argument = sub_matches.get_one::<String>(name);

    match argument {
        Some(name) => name.to_string(),
        _ => "".to_string(),
    }
}

pub fn match_command(matches: &ArgMatches, package_manager: &PackageManager) {
    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Installing {}", argument);
            let executed = package_manager.execute(CommandType::Install, &argument);

            if executed {
                println!("Successfully installed {}", argument)
            }
        }
        Some(("uninstall", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Uninstalling {}", argument)
        }
        Some(("reinstall", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Reinstalling {}", argument)
        }
        Some(("update", _sub_matches)) => {
            print!("Updating repos")
        }
        Some(("upgrade", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Upgrading {}", argument)
        }
        Some(("search", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            print!("Searching for {}", argument)
        }
        Some(("list", _sub_matches)) => {
            print!("Listing all installed packages")
        }
        Some(("clean", _sub_matches)) => {
            print!("Cleaning")
        }
        _ => {
            println!("Invalid command.")
        }
    }
}
