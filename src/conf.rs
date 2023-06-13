use std::path::PathBuf;

use clap::{arg, command, Parser};
use directories::BaseDirs;

pub fn ssh_dir() -> PathBuf {
    BaseDirs::new().unwrap().home_dir().join(".ssh")
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Log level of application
    #[arg(short, long, default_value_t = if cfg!(debug_assertions) { "debug".to_string() } else { "info".to_string() })]
    pub log_level: String,

    /// Path to SSH configuration file `ant.yaml`
    #[arg(short, long, default_value_t = ssh_dir().join("ant.yaml").to_str().unwrap().to_string())]
    pub config: String,

    /// Host to create tunnel for (has to be inside the `hosts` array in `ant.yaml`)
    #[arg()]
    pub host: String,
}
