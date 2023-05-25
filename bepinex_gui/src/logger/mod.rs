use std::fs::File;
use std::sync::Mutex;

use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, Registry};

use crate::data::bepinex_log;

pub fn init() {
    if let Some(log_file_path) = bepinex_log::file::full_path() {
        if let Ok(file) = File::create(log_file_path) {
            let subscriber = Registry::default()
                .with(
                    fmt::Layer::default()
                        .with_writer(Mutex::new(file))
                        .with_ansi(false)
                        .with_line_number(true),
                )
                .with(
                    fmt::Layer::default()
                        .with_writer(std::io::stdout)
                        .with_line_number(true),
                );

            let _ = tracing::subscriber::set_global_default(subscriber);
        } else {
            tracing_subscriber::fmt::init();
        }
    } else {
        tracing_subscriber::fmt::init();
    }
}
