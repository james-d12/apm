mod apm;

use apm::package_manager::PackageManager;
use apm::package_manager::PackageManagement;
use apm::package_manager::command::Command;
use apm::package_manager::command::CommandType;

use apm::managers;

use os_info::Type;

use std::env;
use std::path::Path;


fn arguments(package_manager: PackageManager){
    let env_args = env::args().collect::<Vec<String>>();
    let (bin, _remainer) = env_args.split_first().unwrap();
    let _bin = Path::new(bin).file_stem().unwrap().to_str().unwrap();

    let command_argument: &str = &env_args[1];
    let package_argument: &str;
    
    if env_args.len() - 1 >= 2 {
        package_argument = &env_args[2];
        
        println!("{}", package_argument);
        if package_argument == "" {
            println!("argument 2 is empty!");
            std::process::exit(1);  
        }
    } else {
        package_argument = ""
    }

    println!("Command: {}", package_argument);
    match command_argument {
        "install" => { 
            println!("Install command detected.");
            package_manager.execute(CommandType::Install, package_argument.to_string());
        },
        "reinstall" => {
            println!("Reinstall command detected.");
            package_manager.execute(CommandType::Reinstall, package_argument.to_string());
        },
        "uninstall" => { 
            println!("Uninstall command detected.");
            package_manager.execute(CommandType::Uninstall, package_argument.to_string());
        },
        "update" => { 
            println!("Update command detected.");
            package_manager.execute(CommandType::Update, package_argument.to_string());
        },
        "upgrade" => { 
            println!("Upgrade command detected.");
            package_manager.execute(CommandType::Upgrade, package_argument.to_string());
        },
        "search" => { 
            println!("Search command detected.");
            package_manager.execute(CommandType::Search, package_argument.to_string());
        },
        "list" => {
            println!("List command detected.");
            package_manager.execute(CommandType::List, package_argument.to_string());
        },
        "clean" => { 
            println!("Clean command detected.");
            package_manager.execute(CommandType::Clean, package_argument.to_string());
        },
        "help" => {
            println!("Help command detected.");
            package_manager.execute(CommandType::Help, package_argument.to_string());
        },
        _ => { 
            println!("Command: {} is invalid.", command_argument)

        },
    }
}

fn main() {
    let package_manager: PackageManager;
    match os_info::get().os_type(){
        Type::Alpine => package_manager = managers::apk(),
        Type::Arch => package_manager = managers::pacman(),
        Type::CentOS => package_manager = managers::yum(),
        Type::EndeavourOS => package_manager = managers::apt(),
        Type::Manjaro => package_manager = managers::pacman(),
        Type::Ubuntu => package_manager = managers::apt(),
        Type::Debian => package_manager = managers::apt(),
        Type::Mint => package_manager = managers::apt(),
        Type::Pop => package_manager = managers::apt(),
        Type::Fedora => package_manager = managers::dnf(),
        Type::Redhat => package_manager = managers::yum(),
        Type::openSUSE => package_manager = managers::zypper(),
        Type::Macos => package_manager = managers::brew(),
        Type::Solus => package_manager = managers::eopkg(),
        Type::Windows => package_manager = managers::choco(),
        _ => {
            println!("Cannot find appropriate package manager!");
            std::process::exit(1);
        },
    }    
    package_manager.print();
    arguments(package_manager);
}
 