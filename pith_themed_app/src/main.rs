#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::redundant_locals,
    dead_code
)]

mod app;
mod pages;
mod theme;

use leptos::prelude::*;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
