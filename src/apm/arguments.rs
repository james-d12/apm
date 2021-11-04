use clap::{Arg, App};

pub struct Argument {
    pub command_argument: String,
    pub package_argument: String
}
 
fn get_matches() -> clap::ArgMatches {
    let matches = App::new("Agnostic Package Manager")
        .version("0.1")
        .author("James Durban")
        .about("Manages packages.")
        .arg(Arg::new("COMMAND")
            .about("The Command to run")
            .required(true)
            .index(1))
        .arg(Arg::new("PACKAGE")
            .about("The Package to install.")
            .required(false)
            .index(2))
        .get_matches();
    return matches;
}

fn process_arguments(matches: &clap::ArgMatches) -> Argument {
    let mut command_argument: String = String::new();
    let mut package_argument: String = String::new();
    
    let valid_commands = [ "install", "uninstall", "update", "upgrade", "list", "search" ];
    let commands_requiring_install = [ "install", "uninstall", "search" ];

    if let Some(i) = matches.value_of("COMMAND") {
        let lower = i.to_lowercase();
        let formatted = lower.trim();
        command_argument = formatted.to_string();

        
        if !valid_commands.contains(&formatted){
            println!("Invalid command argument: {}.", &formatted);
            std::process::exit(1);
        }

        if commands_requiring_install.contains(&formatted) {
            if let Some(i) = matches.value_of("PACKAGE") {
                let lower = i.to_lowercase();
                let formatted = lower.trim();
                package_argument = formatted.to_string();
            }
        }
    }

    return Argument {
        command_argument: command_argument,
        package_argument: package_argument
    }
}

pub fn get_arguments() -> Argument {
    return process_arguments(&get_matches());
}