use std::fs::File;
use std::sync::Mutex;

use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, Registry};

use crate::data::bepinex_log;

pub fn init() {
    bepinex_log::file::full_path().map_or_else(
        || {
            tracing_subscriber::fmt::init();
        },
        |log_file_path| {
            File::create(log_file_path).map_or_else(
                |_err| {
                    tracing_subscriber::fmt::init();
                },
                |file| {
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
                },
            )
        },
    )
}
