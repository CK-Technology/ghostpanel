mod app;
mod auth;
mod components;
mod pages;
mod services;
mod utils;

use leptos::*;
use wasm_bindgen::prelude::*;

pub use app::App;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}