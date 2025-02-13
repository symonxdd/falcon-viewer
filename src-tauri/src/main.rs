// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    falkon_viewer_lib::run() // Calls the `run` function from lib.rs
}
