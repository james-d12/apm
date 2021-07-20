mod apm;

use apm::package_manager::PackageManager;
use apm::package_manager::PackageManagement;
use apm::package_manager::command::Command;
use apm::package_manager::command::CommandType;

use apm::managers;

use os_info;

fn main() {
    let info = os_info::get();

    // Print full information:
    println!("OS information: {}", info);
    
    // Print information separately:
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
    println!("Edition: {:?}", info.edition());
    println!("Codename: {:?}", info.codename());
    println!("Bitness: {}", info.bitness());

    managers::apt().print();
    managers::choco().print();
    managers::dnf().print();
    managers::pacman().print();
    managers::yum().print();
}
 