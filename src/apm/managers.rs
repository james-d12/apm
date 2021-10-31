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

fn apk() -> PackageManager { return apk::get_apk_manager(); }
fn apt() -> PackageManager { return apt::get_apt_manager(); }
fn brew() -> PackageManager { return brew::get_brew_manager(); }
fn choco() -> PackageManager { return choco::get_choco_manager(); }
fn dnf() -> PackageManager { return dnf::get_dnf_manager(); }
fn eopkg() -> PackageManager { return eopkg::get_eopkg_manager(); }
fn pacman() -> PackageManager { return pacman::get_pacman_manager(); }
fn yum() -> PackageManager { return yum::get_yum_manager(); }
fn zypper() -> PackageManager { return zypper::get_zypper_manager(); }

pub fn decide_package_manager() -> Option<PackageManager> {
    let package_manager: PackageManager;
    match os_info::get().os_type(){
        Type::Alpine => package_manager = apk(),
        Type::Arch => package_manager = pacman(),
        Type::CentOS => package_manager = yum(),
        Type::EndeavourOS => package_manager = apt(),
        Type::Manjaro => package_manager = pacman(),
        Type::Ubuntu => package_manager = apt(),
        Type::Debian => package_manager = apt(),
        Type::Mint => package_manager = apt(),
        Type::Pop => package_manager = apt(),
        Type::Fedora => package_manager = dnf(),
        Type::Redhat => package_manager = yum(),
        Type::openSUSE => package_manager = zypper(),
        Type::Macos => package_manager = brew(),
        Type::Solus => package_manager = eopkg(),
        Type::Windows => package_manager = choco(),
        _ => { return None },
    }
    return Some(package_manager);
}