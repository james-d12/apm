# Agnostic Package Manager
[![Build](https://github.com/james-d12/Apm/actions/workflows/ci.yml/badge.svg)](https://github.com/james-d12/Apm/actions/workflows/ci.yml)

APM aims to be a multipurpose multiplatform package manager that wraps around some of the most common package managers for various distributions and platforms.

# What does it do?
It is essentially a wrapper around other package managers. It autodetects the platform you are using and sets the package manager accordingly. After that all commands are then run using that package manager's appropriate command.


# List of supported Operating Systems and Distributions.
APM uses [os_info](https://github.com/stanislav-tkach/os_info) to get the current operating system and distribution. As a result it supports the most of the same stuff that os_info does, as long as I have added in the appropriate package manager [here](https://github.com/james-d12/Apm/tree/master/src/apm/managers)

# Examples

- ```apm install {package}``` Will install the given package, no need to remember what the install command is anymore! 
- ```apm uninstall {package}``` Will uninstall the given package.
- ```apm update``` Will update all package repositories.
- ```apm upgrade``` Will update and upgrade all packages.
- ```apm search {package}``` Will search for the given package.
- ```apm list``` Will list installed packages.

# Coming features

- [ ] Support for more detailed arguments. For example some commands like 'update' have extra arguments to say update one package or all packages. 
- [ ] Support for NPM and Local package management, alongside managing operating system packages.
- [ ] Extensive testing.

## Package Managers

| Package Manager  | Install | Uninstall | Update | Upgrade | Search | List |
| ---------------- | ---- | --- | --- | --- | --- | --- |
| **Apt**          | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  |
| **Apk**          | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  |
| **Yum**          | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  |
| **Dnf**          | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  |
| **Pacman**       | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  |
| **Zypper**       | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  |
| **Eopkg**        | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  |
| **Choco**        | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  |
| **Brew**         | ✔️  | ✔️  | ✔️  | ✔️ | ✔️ | ✔️  | 
| **NPM**          | ❌  | ❌  | ❌  | ❌ | ❌ | ❌  | 
| **Conan**        | ❌  | ❌  | ❌  | ❌ | ❌ | ❌  | 
