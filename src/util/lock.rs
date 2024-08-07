use std::{fs, fs::File};
use std::path::Path;
use crate::util::macros::continue_prompt;

/// Creates a lock file indicating that ps4 is open
pub fn create_lock() -> std::io::Result<()> {
    File::create("/tmp/ps4.hacking")?;
    Ok(())
}

/// Deletes the lock file
pub fn remove_lock() -> std::io::Result<()>{
    fs::remove_file("/tmp/ps4.hacking")?;
    Ok(())
}

/// Returns true if the lock file exists on the file system
pub fn check_lock() -> bool {
    Path::new("/tmp/ps4.hacking").exists()
}

/// Check if a ps4 instance is already running and give the option of removing the lock file
pub fn lock_exists() {
    if check_lock() {
        println!("An instance of ps4 is already running.");
        println!("Delete lock file? (Only do this when the other process is frozen)");
        if continue_prompt() {
            remove_lock().expect("Failed to delete lock file.");
        } else {
            std::process::exit(1);
        }
    }
}