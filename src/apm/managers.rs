mod apk;
mod apt;
mod brew;
mod choco;
mod dnf;
mod eopkg;
mod pacman;
mod yum;
mod zypper;

use crate::PackageManager;
use os_info::Type;

pub fn decide_package_manager() -> Option<PackageManager> {
    let package_manager: PackageManager;
    match os_info::get().os_type() {
        Type::Alpine => package_manager = apk::apk(),
        Type::Arch => package_manager = pacman::pacman(),
        Type::CentOS => package_manager = yum::yum(),
        Type::EndeavourOS => package_manager = apt::apt(),
        Type::Manjaro => package_manager = pacman::pacman(),
        Type::Ubuntu => package_manager = apt::apt(),
        Type::Debian => package_manager = apt::apt(),
        Type::Mint => package_manager = apt::apt(),
        Type::Pop => package_manager = apt::apt(),
        Type::Fedora => package_manager = dnf::dnf(),
        Type::Redhat => package_manager = yum::yum(),
        Type::openSUSE => package_manager = zypper::zypper(),
        Type::Macos => package_manager = brew::brew(),
        Type::Solus => package_manager = eopkg::eopkg(),
        Type::Windows => package_manager = choco::choco(),
        _ => return None,
    }
    return Some(package_manager);
}
