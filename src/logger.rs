use std::io::Write;

use chrono::Local;
use colored::*;
use env_logger::{self, Logger};
use log::Level;

pub fn build_custom_env_logger() -> Logger {
    env_logger::builder()
        .format(|buf, record| {
            let level = record.level();
            let level = match level {
                Level::Trace => level.as_str().purple(),
                Level::Debug => level.as_str().blue(),
                Level::Info => level.as_str().green(),
                Level::Warn => level.as_str().yellow(),
                Level::Error => level.as_str().red(),
            };
            // let prefix = format!(
            //     "{} {:5} {}:{} -",
            //     Local::now().format("%X"),
            //     level,
            //     record.file_static().unwrap(),
            //     record.line().unwrap(),
            // );
            let prefix = format!(
                "{} {:5} {} -",
                Local::now().format("%X"),
                level,
                record.target(),
            );

            writeln!(buf, "{} {}", prefix.bold(), record.args())
        })
        .format_indent(Some(4))
        .build()
}
