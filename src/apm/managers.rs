mod apk;
mod apt;
mod brew;
mod choco;
mod dnf;
mod eopkg;
mod pacman;
mod yum;
mod zypper;

use os_info::Type;
use crate::PackageManager;

pub fn decide_package_manager() -> Option<PackageManager> {
    let package_manager: PackageManager;
    match os_info::get().os_type(){
        Type::Alpine => package_manager = apk::get_apk(),
        Type::Arch => package_manager = pacman::get_pacman(),
        Type::CentOS => package_manager = yum::get_yum(),
        Type::EndeavourOS => package_manager = apt::get_apt(),
        Type::Manjaro => package_manager = pacman::get_pacman(),
        Type::Ubuntu => package_manager = apt::get_apt(),
        Type::Debian => package_manager = apt::get_apt(),
        Type::Mint => package_manager = apt::get_apt(),
        Type::Pop => package_manager = apt::get_apt(),
        Type::Fedora => package_manager = dnf::get_dnf(),
        Type::Redhat => package_manager = yum::get_yum(),
        Type::openSUSE => package_manager = zypper::get_zypper(),
        Type::Macos => package_manager = brew::get_brew(),
        Type::Solus => package_manager = eopkg::get_eopkg(),
        Type::Windows => package_manager = choco::get_choco(),
        _ => { return None },
    }
    return Some(package_manager);
}