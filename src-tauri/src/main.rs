// Hides the extra console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = sallygate_lib::run() {
        eprintln!("Sallygate failed to start: {error}");
        std::process::exit(1);
    }
}
