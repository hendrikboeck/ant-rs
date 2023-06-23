use log::error;
use std::{path::PathBuf, process::exit};

use clap::{arg, command, Parser};
use directories::BaseDirs;

use once_cell::sync::Lazy;

pub static TMP_DIR: Lazy<PathBuf> = Lazy::new(tmp_dir);

#[cfg(not(target_os = "windows"))]
fn tmp_dir() -> PathBuf {
    PathBuf::from("/tmp/ant")
}

#[cfg(target_os = "windows")]
fn tmp_dir() -> PathBuf {
    BaseDirs::new()
        .unwrap_or_else(|| {
            error!("Failed to fetch BaseDirs from crate `directories`");
            exit(exitcode::OSERR);
        })
        .data_local_dir()
        .join("Temp")
        .join("ant")
}

#[inline]
fn default_ant_config() -> String {
    BaseDirs::new()
        .unwrap_or_else(|| {
            error!("Failed to fetch BaseDirs from crate `directories`");
            exit(exitcode::OSERR);
        })
        .home_dir()
        .join(".ssh")
        .join("ant.yaml")
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg(debug_assertions)]
#[inline]
fn default_log_level() -> String {
    "debug".into()
}

#[cfg(not(debug_assertions))]
#[inline]
fn default_log_level() -> String {
    "info".into()
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
pub struct CliArgs {
    /// Log level of application
    #[arg(short, long, default_value_t = default_log_level())]
    pub log_level: String,

    /// Path to ANT configuration file `ant.yaml`
    #[arg(short, long, default_value_t = default_ant_config())]
    pub config: String,

    /// Run application in daemon mode. (requires `autossh` to work)
    #[arg(short, long, action)]
    pub daemon: bool,

    /// Host to create tunnel(s) for. Has to be in `hosts` inside your ANT configuration file.
    /// This host has to accessible without user input. (use identity_file, etc. for authentication).
    #[arg()]
    pub host: String,
}
