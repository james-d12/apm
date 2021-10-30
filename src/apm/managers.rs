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

pub fn apk() -> PackageManager { return apk::get_apk_manager(); }
pub fn apt() -> PackageManager { return apt::get_apt_manager(); }
pub fn brew() -> PackageManager { return brew::get_brew_manager(); }
pub fn choco() -> PackageManager { return choco::get_choco_manager(); }
pub fn dnf() -> PackageManager { return dnf::get_dnf_manager(); }
pub fn eopkg() -> PackageManager { return eopkg::get_eopkg_manager(); }
pub fn pacman() -> PackageManager { return pacman::get_pacman_manager(); }
pub fn yum() -> PackageManager { return yum::get_yum_manager(); }
pub fn zypper() -> PackageManager { return zypper::get_zypper_manager(); }