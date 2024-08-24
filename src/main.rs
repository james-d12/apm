mod apm;

use crate::apm::terminal::check_executable_exists;
use apm::cli;
use apm::command::Command;
use apm::command::CommandType;
use apm::package_manager::PackageManagement;
use apm::package_manager::PackageManager;

fn main() {
    let package_manager = apm::managers::decide_package_manager();

    match package_manager {
        Some(package_manager) => {
            let executable_exists = check_executable_exists(&package_manager.package_name);

            match executable_exists {
                true => {
                    let matches = cli::get_cli().get_matches();
                    cli::run_cli(&matches, &package_manager);
                }
                false => {
                    println!(
                        "Could not find executable: {0} for package manager: {1}.",
                        &package_manager.package_name, &package_manager.name
                    );
                    std::process::exit(1);
                }
            }
        }
        None => {
            println!("Could not find appropriate package manager for current system.");
            std::process::exit(1);
        }
    }
}
