//!---

use crate::{config::Config, utils::LogType};

mod app_state;
mod config;
mod config_dir;
mod error;
mod prelude;
mod project;
mod root_view;
mod template;
mod theme;
mod utils;

fn main() {
    let config = Config::load().unwrap_or_else(|err| {
        utils::log(&err.to_string(), LogType::Error);

        Config::default()
    });

    utils::run_app(config);
}
