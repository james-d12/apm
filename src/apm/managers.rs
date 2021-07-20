mod apt;
mod choco;
mod dnf;
mod pacman;
mod yum;

use crate::PackageManager;

pub fn apt() -> PackageManager { return apt::get_apt_manager(); }
pub fn choco() -> PackageManager { return choco::get_choco_manager(); }
pub fn dnf() -> PackageManager { return dnf::get_dnf_manager(); }
pub fn pacman() -> PackageManager { return pacman::get_pacman_manager(); }
pub fn yum() -> PackageManager { return yum::get_yum_manager(); }


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_apt() {
        let apt: PackageManager = apt();
        assert_eq!(apt.name, "Aptitude Package Manager");
        assert_eq!(apt.package_name, "apt-get");
    }

}