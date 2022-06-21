mod apm;

use apm::cli;
use apm::command::Command;
use apm::command::CommandType;
use apm::package_manager::PackageManagement;
use apm::package_manager::PackageManager;

fn main() {
    let package_manager: Option<PackageManager> = apm::managers::decide_package_manager();

    match package_manager {
        Some(package_manager) => {
            let matches = cli::get_cli().get_matches();
            let executed = cli::match_command(&matches, &package_manager);

            match executed {
                true => {
                    println!("Finished running command.")
                }
                _ => {
                    println!("Failed running command.")
                }
            }
        }
        None => {
            println!("Could not find appropriate package manager for current system.");
            std::process::exit(1);
        }
    }
}
