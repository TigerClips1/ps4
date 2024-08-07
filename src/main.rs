use std::env;

mod commands;
mod util;

/// Get a static string of the current ps4 version
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if any command was supplied
    if args.len() < 2 {
        commands::help::help();
        std::process::exit(0);
    }

    let command: String = args[1].to_lowercase();

    match &command[..] {
        // Help commands    
        "-h" => commands::help::help(),
        "--help" => commands::help::help(),

        // Sync commands
        "S" => commands::sync::sync(),
        "sync" => commands::sync::sync(),

        // Upgrade commands
        "U" => commands::upgrade::upgrade(),
        "upgrade" => commands::upgrade::upgrade(),

        // Install commands
        "I" => commands::install::install(args),
        "install" => commands::install::install(args),
        "LI" => commands::localinstall::local_install(args),
        "localinstall" => commands::localinstall::local_install(args),
        "GI" => commands::groupinstall::group_install(args),
        "groupinstall" => commands::groupinstall::group_install(args),

        // Remove commands
        "u" => commands::remove::remove(args),
        "uninstall" => commands::remove::remove(args),

        // Info commands

        // List commands
        "list" => commands::list::list(),

        // Specify that command is invalid and show help command
        _ => {
            println!("ps4: Invalid command \"{}\", use {{-h --help}} for valid commands.", command);
        }
    }
}
