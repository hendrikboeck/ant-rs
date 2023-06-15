use chrono::Local;
use colored::*;
use env_logger::{self, Builder};
use log::Level;
use std::io::Write;

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
            //let prefix = format!(
            //    "{} {:5} {}:{} -",
            //    Local::now().format("%X"),
            //    level,
            //    record.file_static().unwrap(),
            //    record.line().unwrap(),
            //);
            let prefix = format!(
                "{} {:5} {} -",
                Local::now().format("%X"),
                level,
                record.target(),
            );

            writeln!(buf, "{} {}", prefix.bold(), record.args())
        })
        .format_indent(Some(4));

    builder
}
