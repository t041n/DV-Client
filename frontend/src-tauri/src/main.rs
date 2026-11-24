// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let exit_code = dv_core::dv_run();
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
