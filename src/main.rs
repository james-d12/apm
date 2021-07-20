mod apm;

use apm::package_manager::PackageManager;
use apm::package_manager::PackageManagement;
use apm::package_manager::command::Command;
use apm::package_manager::command::CommandType;

use apm::managers;

fn main() {
    managers::apt().print();
    managers::choco().print();
    managers::dnf().print();
    managers::pacman().print();
    managers::yum().print();
}
 