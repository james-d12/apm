mod apm;

use apm::cli;
use apm::command::Command;
use apm::command::CommandType;
use apm::package_manager::PackageManagement;
use apm::package_manager::PackageManager;

fn main() {
    let package_manager = apm::managers::decide_package_manager();

    match package_manager {
        Some(package_manager) => {
            let matches = cli::get_cli().get_matches();
            cli::run_cli(&matches, &package_manager);
        }
        None => {
            println!("Could not find appropriate package manager for current system.");
            std::process::exit(1);
        }
    }
}
