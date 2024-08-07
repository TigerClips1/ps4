pub fn help() {
    println!("ps4 - the JaguarLinux package manger - v{}", crate::get_version());
    println!("usage: ps4 <command> [...]");
    println!("commands:");
    println!("\t ps4 {{-h --help}}");
    println!("\t\t - List all commands for ps4 (this view)");
    println!("\t ps4 {{s sync}}");
    println!("\t\t - Synchronizes package databases with remotes");
    println!("\t ps4 {{u upgrade}}");
    println!("\t\t - Check for (and then install) package updates");
    println!("\t ps4 {{i install}} <package(s)>");
    println!("\t\t - Install a specified package");
    println!("\t ps4 {{li localinstall}} <path(s)>");
    println!("\t\t - Install a package from a local archive");
    println!("\t ps4 {{r remove}} <package(s)>");
    println!("\t\t - Uninstall a specified package");
    //println!("\t ps4info <package>");
    //println!("\t\t - TODO");
    //println!("\t ps4 search <package>");
    //println!("\t\t - TODO");
    println!("\t ps4 list");
    println!("\t\t - List all installed packages with their version and source");
}