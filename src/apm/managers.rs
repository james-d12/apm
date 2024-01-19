mod apk;
mod apt;
mod brew;
mod choco;
mod dnf;
mod eopkg;
mod npm;
mod pacman;
mod winget;
mod yum;
mod zypper;

use crate::PackageManager;
use os_info::Type;

pub fn decide_package_manager() -> Option<PackageManager> {
    let package_manager = match os_info::get().os_type() {
        Type::Alpine => apk::apk(),
        Type::Arch => pacman::pacman(),
        Type::CentOS => yum::yum(),
        Type::EndeavourOS => apt::apt(),
        Type::Manjaro => pacman::pacman(),
        Type::Ubuntu => apt::apt(),
        Type::Debian => apt::apt(),
        Type::Mint => apt::apt(),
        Type::Pop => apt::apt(),
        Type::Fedora => dnf::dnf(),
        Type::Redhat => yum::yum(),
        Type::openSUSE => zypper::zypper(),
        Type::Macos => brew::brew(),
        Type::Solus => eopkg::eopkg(),
        Type::Windows => winget::winget(),
        _ => return None,
    };
    Some(package_manager)
}
