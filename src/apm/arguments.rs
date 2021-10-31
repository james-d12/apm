use std::env;
use std::path::Path;

pub struct Argument {
    pub command_argument: String,
    pub package_argument: String
}

pub fn process_arguments() -> Argument {
    let env_args = env::args().collect::<Vec<String>>();
    let (bin, _remainer) = env_args.split_first().unwrap();
    let _bin = Path::new(bin).file_stem().unwrap().to_str().unwrap();

    if env_args.len() < 2 {
        println!("No command provided.");
        std::process::exit(1);
    }

    let command_argument: &str = &env_args[1];
    let package_argument: &str;

    if env_args.len() - 1 >= 2 { package_argument = &env_args[2] } else { package_argument = "" }

    return Argument {
        command_argument: command_argument.to_string(),
        package_argument: package_argument.to_string()
    };
}