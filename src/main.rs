mod conf;
mod logger;
mod yaml;

use clap::Parser;
use colored::*;
use exitcode;
use log::{debug, error, info};
use openssh::{ForwardType, KnownHosts, SessionBuilder};
use std::{
    env, fs,
    net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr},
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::conf::{ssh_dir, CliArgs};
use crate::logger::build_custom_env_logger;
use crate::yaml::Root;

#[tokio::main]
async fn main() {
    const PKG_NAME: &str = env!("CARGO_PKG_NAME");
    const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

    let pname = format!(
        "🐜 {} {} - Command Line Tool",
        PKG_NAME.to_uppercase().bold(),
        PKG_VERSION
    );
    let borders = String::from_iter(['-'; 50]).bold();
    println!("{}\n{}\n{}", borders, pname, borders);

    let args = CliArgs::parse();
    env::set_var("RUST_LOG", &args.log_level);
    if args.log_level.to_lowercase() == "debug" && env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let logger = build_custom_env_logger();
    let max_level = logger.filter();
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(max_level);

    info!("Parsed arguments from CLI");
    debug!("{:#?}", &args);

    let file = fs::read_to_string(&args.config).unwrap_or_else(|e| {
        error!("Failed load file `{}`", &args.config);
        error!("Original Error: {}", e.to_string().italic());
        exit(exitcode::IOERR);
    });
    info!("Loaded SSH host configuration from `{}`", &args.config);

    let root: Root = serde_yaml::from_str(&file).unwrap_or_else(|e| {
        error!(
            "Data in `{}` was malformed, please check documentation.",
            &args.config
        );
        error!("Original Error: {}", e.to_string().italic());
        exit(exitcode::CONFIG);
    });
    debug!("{:#?}", &root);

    if root.version != PKG_VERSION {
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

    let se = SessionBuilder::default()
        .connect_timeout(Duration::from_secs(10))
        .user(host.user.clone())
        .keyfile(ssh_dir().join(&host.identity_file))
        .port(host.port)
        .compression(true)
        .known_hosts_check(KnownHosts::Strict)
        .connect(&host.hostname)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to connect to host `{}`", &args.host);
            error!("Original Error: {}", e.to_string().italic());
            exit(exitcode::NOHOST);
        });

    for fwd in host.local_forward.iter() {
        let local = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), fwd.local);
        let remote: SocketAddr = fwd.remote.parse().unwrap_or_else(|e: AddrParseError| {
            error!("Malformed ip address and port");
            error!("Original Error: {}", e.to_string().italic());
            exit(exitcode::CONFIG);
        });
        se.request_port_forward(ForwardType::Local, local, remote)
            .await
            .unwrap_or_else(|e| {
                error!(
                    "Port forward request failed for {} -> localhost:{}",
                    &fwd.remote, fwd.local
                );
                error!("Original Error: {}", e.to_string().italic());
                exit(exitcode::UNAVAILABLE);
            });
        info!("Forwarding {} to localhost:{}", &fwd.remote, fwd.local);
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || r.store(false, Ordering::SeqCst)).unwrap_or_else(|e| {
        error!("Failed to spawn Ctrl-C interrupt");
        error!("Original Error: {}", e.to_string().italic());
        exit(exitcode::OSERR);
    });

    info!("Press Ctrl-C to close tunnel");
    while running.load(Ordering::SeqCst) {}
    println!();
    info!("Closing connection to host `{}`", &args.host);
    se.close().await.unwrap();
}
