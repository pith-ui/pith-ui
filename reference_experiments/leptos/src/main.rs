#![allow(
    clippy::clone_on_copy,
    clippy::redundant_locals,
    clippy::type_complexity
)]

mod app;
mod pages;

use leptos::prelude::*;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
