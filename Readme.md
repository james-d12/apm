# Agnostic Package Manager
[![Build](https://github.com/james-d12/Apm/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/james-d12/Apm/actions/workflows/ci.yml)

APM aims to be a multipurpose multiplatform package manager that wraps around some of the most common package managers for various distributions and platforms, making managing packages easier and less hassle.

# What does it do?
It is essentially a wrapper around other package managers. It autodetects the platform you are using and sets the package manager accordingly. After that all commands are then run using that package manager's appropriate command.


# List of supported Operating Systems and Distributions.
APM uses [os_info](https://github.com/stanislav-tkach/os_info) to get the current operating system and distribution. As a result it supports the most of the same stuff that os_info does, as long as I have added in the appropriate package manager [here](https://github.com/james-d12/Apm/tree/master/src/apm/managers)

# Commands

Apm takes in commands in the following format: ```apm {COMMAND} {PACKAGE} {OPTIONS}```. Each is optional, with some commands requiring a package such as install, uninstall, etc.

| Command          | Function                        | Requires Package  |
| ---------------- | ------------------------------- | ----------------- |
| ```install```    | Installs a given package.       | ✔️ |
| ```uninstall```  | Uninstalls a given package.     | ✔️ |
| ```update```     | Updates package sources.        | ❌|
| ```upgrade```     | Upgrades packages to latest version. |❌| 
| ```search```     | Searches for a given package (remote & local). | ✔️ |
| ```list```     | Lists all installed packages (local). |❌|
| ```clean```     | Cleans cache for package manager. |❌|
| ```help```     | Prints the help for the underlying package manager. |❌|

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
