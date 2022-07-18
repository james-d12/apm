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
                .alias("i")
                .about("installs a provided package.")
                .arg(arg!(<PACKAGE> "The package to install"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("uninstall")
                .alias("u")
                .about("Uninstalls a provided package.")
                .arg(arg!(<PACKAGE> "The package to uninstall"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("reinstall")
                .alias("r")
                .about("Reinstalls a provided package.")
                .arg(arg!(<PACKAGE> "The package to reinstall"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("update")
                .alias("ud")
                .about("Updates repositories."),
        )
        .subcommand(
            Command::new("upgrade")
                .alias("ug")
                .about("Upgrades a package to latest version.")
                .arg(arg!(<PACKAGE> "The package to upgrade."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("search")
                .alias("s")
                .about("Searches for a package.")
                .arg(arg!(<PACKAGE> "The package to search for"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("Lists all installed packages."),
        )
        .subcommand(
            Command::new("outdated")
                .alias("od")
                .about("Lists all outdated packages."),
        )
        .subcommand(
            Command::new("clean")
                .alias("c")
                .about("Cleans package manager's cache"),
        )
        .subcommand(
            Command::new("info")
                .alias("if")
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

pub fn run_cli(matches: &ArgMatches, package_manager: &PackageManager) -> bool {
    let mut result = true;
    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            result = package_manager.execute(CommandType::Install, &argument, "Installing");
        }
        Some(("uninstall", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            result = package_manager.execute(CommandType::Uninstall, &argument, "Uninstalling");
        }
        Some(("reinstall", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            result = package_manager.execute(CommandType::Reinstall, &argument, "Reinstalling");
        }
        Some(("update", _sub_matches)) => {
            result = package_manager.execute(CommandType::Update, "", "Updating");
        }
        Some(("upgrade", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            result = package_manager.execute(CommandType::Upgrade, &argument, "Upgrading");
        }
        Some(("search", sub_matches)) => {
            let argument = get_argument(sub_matches, "PACKAGE");
            result = package_manager.execute(CommandType::Search, &argument, "Searching for");
        }
        Some(("list", _sub_matches)) => {
            result = package_manager.execute(CommandType::List, "", "Listing installed packages");
        }
        Some(("outdated", _sub_matches)) => {
            result =
                package_manager.execute(CommandType::Outdated, "", "Listing outdated packages");
        }
        Some(("clean", _sub_matches)) => {
            result = package_manager.execute(CommandType::Clean, "", "Cleaning");
        }
        Some(("info", _sub_matches)) => package_manager.print(),
        _ => {
            println!("Invalid command.");
        }
    }
    return result;
}
