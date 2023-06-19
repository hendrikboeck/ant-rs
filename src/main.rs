mod command_builder;
mod configuration;
mod logger;
mod yaml;

use clap::Parser;
use colored::*;
use exitcode;
use log::{debug, error, info, warn};
use std::{
    env, fs,
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::command_builder::{build_ssh_process, LOG_FILE};
use crate::configuration::{CliArgs, TMP_DIR};
use crate::logger::build_custom_env_logger;
use crate::yaml::Root;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const PKG_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const RUST_BACKTRACE: &str = "RUST_BACKTRACE";
const RUST_LOG: &str = "RUST_LOG";

#[tokio::main]
async fn main() {
    let pname = format!(
        "🐜 {} {} - Command Line Tool",
        PKG_NAME.to_uppercase().bold(),
        PKG_VERSION
    );
    let borders = String::from_iter(['-'; 50]).bold();
    println!("{}\n{}\n{}", borders, pname, borders);

    let args = CliArgs::parse();
    env::set_var(RUST_LOG, &args.log_level);
    if args.log_level.to_lowercase() == "debug" && env::var(RUST_BACKTRACE).is_err() {
        env::set_var(RUST_BACKTRACE, "1");
    }

    build_custom_env_logger().init();

    info!("Parsed arguments from CLI");
    debug!("{:#?}", &args);

    if !TMP_DIR.exists() {
        fs::create_dir_all(TMP_DIR.as_path()).unwrap_or_else(|e| {
            error!(
                "Failed to create local directory `{}`. Does not yet exist in filepath.",
                TMP_DIR.to_string_lossy()
            );
            error!("Original Error: {}", e.to_string().italic());
            exit(exitcode::SOFTWARE);
        });
    }

    let file = fs::read_to_string(&args.config).unwrap_or_else(|e| {
        error!("Failed load file `{}`", &args.config);
        error!("Original Error: {}", e.to_string().italic());
        exit(exitcode::IOERR);
    });
    info!("Loaded ant configuration from `{}`", &args.config);

    let root: Root = serde_yaml::from_str(&file).unwrap_or_else(|e| {
        error!(
            "Data in `{}` was malformed, please check documentation.",
            &args.config
        );
        error!("Original Error: {}", e.to_string().italic());
        exit(exitcode::CONFIG);
    });
    debug!("{:#?}", &root);

    let root_pkg_version: Vec<&str> = root.version.split('.').collect();
    if root_pkg_version[0] != PKG_VERSION_MAJOR || root_pkg_version[1] != PKG_VERSION_MINOR {
        error!(
            "Application version did not match version found in `{}`: expected {:?}, found {:?}",
            &args.config, &PKG_VERSION, &root.version
        );
        exit(exitcode::CONFIG);
    }

    let host = root.hosts.get(&args.host).unwrap_or_else(|| {
        error!(
            "Host `{}` could not be found in `{}`",
            &args.host, &args.config
        );
        exit(exitcode::CONFIG);
    });
    info!("Loaded configuration for host `{}`", &args.host);
    debug!("{:#?}", &host);

    if host.local_forward.is_none() && host.remote_forward.is_none() {
        error!("At least one forwarding option has be set (`local_forward` or `remote_forward`)");
        exit(exitcode::CONFIG);
    }

    let mut proc = build_ssh_process(&args.host, &host)
        .spawn()
        .unwrap_or_else(|e| {
            error!("Failed to spawn SSH process");
            error!("Original Error: {}", e.to_string().italic());
            exit(exitcode::OSERR);
        });
    info!("Spawned SSH process with PID {}", proc.id().unwrap());

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || r.store(false, Ordering::SeqCst)).unwrap_or_else(|e| {
        error!("Failed to spawn Ctrl-C interrupt");
        error!("Original Error: {}", e.to_string().italic());
        exit(exitcode::OSERR);
    });

    info!("Waiting on Ctrl-C...");
    while running.load(Ordering::SeqCst) {
        if let Some(status) = proc.try_wait().unwrap_or_else(|e| {
            warn!(
                "unwrap() on process status failed; error: {}",
                e.to_string().italic()
            );
            None
        }) {
            error!(
                "Process exited prematurely with code {}, please check logs at `{}` for futher diagnosis.",
                status.code().unwrap(),
                LOG_FILE.get().unwrap()
            );
            exit(exitcode::SOFTWARE);
        }
    }
    println!();
    info!("Killing SSH process with PID {}", proc.id().unwrap());
    proc.kill().await.unwrap();
    info!("Finished. Exiting...");
}
