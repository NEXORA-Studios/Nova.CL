#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![feature(async_trait_bounds, derive_from)] // Enable unstable features

fn main() {
    novacl_lib::run();
}
