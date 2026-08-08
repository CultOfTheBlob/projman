//!---

use crate::{config::Config, log::Log};

mod app_state;
mod config;
mod config_dir;
mod error;
mod log;
mod prelude;
mod project;
mod root_view;
mod template;
mod theme;
mod utils;

fn main() {
    let config = Config::load().unwrap_or_else(|err| {
        Log::Error.log(&err.to_string());

        Config::default()
    });

    utils::run_app(config);
}
