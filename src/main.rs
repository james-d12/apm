mod apm;

use apm::package_manager::PackageManager;
use apm::package_manager::PackageManagement;
use apm::package_manager::command::Command;
use apm::package_manager::command::CommandType;

use apm::managers;

use os_info;
use os_info::Type;

fn main() {
    let info = os_info::get();

    // Print full information:
    println!("OS information: {}", info);

    let package_manager: PackageManager;
    match info.os_type(){
        Type::Alpine => package_manager = managers::apk(),
        Type::Arch => package_manager = managers::pacman(),
        Type::CentOS => package_manager = managers::yum(),
        Type::EndeavourOS => package_manager = managers::apt(),
        Type::Manjaro => package_manager = managers::pacman(),
        Type::Ubuntu => package_manager = managers::apt(),
        Type::Debian => package_manager = managers::apt(),
        Type::Mint => package_manager = managers::apt(),
        Type::Pop => package_manager = managers::apt(),
        Type::Fedora => package_manager = managers::dnf(),
        Type::Redhat => package_manager = managers::yum(),
        Type::openSUSE => package_manager = managers::zypper(),
        Type::Macos => package_manager = managers::brew(),
        Type::Windows => package_manager = managers::choco(),
        _ => {
            println!("Cannot find appropriate package manager!");
            std::process::exit(1);
        },
    }

    package_manager.print();
}
 