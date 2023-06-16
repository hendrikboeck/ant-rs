use chrono::Local;
use colored::*;
use env_logger::{self, Builder};
use log::Level;
use std::io::Write;

const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn build_custom_env_logger() -> Builder {
    let mut builder = env_logger::builder();

    builder
        .format(|buf, record| {
            let level = record.level();
            let level = match level {
                Level::Trace => level.as_str().purple(),
                Level::Debug => level.as_str().blue(),
                Level::Info => level.as_str().green(),
                Level::Warn => level.as_str().yellow(),
                Level::Error => level.as_str().red(),
            };
            let prefix = format!(
                "{} {:5} com.{}.cli -",
                Local::now().format("%X"),
                level,
                CARGO_PKG_NAME
            );

            writeln!(buf, "{} {}", prefix.bold(), record.args())
        })
        .format_indent(Some(4));

    builder
}
