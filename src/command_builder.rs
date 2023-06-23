use colored::Colorize;
use log::{debug, info, warn};
use once_cell::sync::OnceCell;
use std::process::Stdio;
use tokio::process::Command;

use chrono::Local;

use crate::configuration::TMP_DIR;
use crate::yaml::{Host, SshOptions};

pub static LOG_FILE: OnceCell<String> = OnceCell::new();

fn format_ip(addr_or_port: &String) -> String {
    if addr_or_port.contains(':') {
        addr_or_port.clone()
    } else {
        format!("localhost:{}", addr_or_port)
    }
}

pub fn build_ssh_process(name: &str, host: &Host) -> Command {
    let ssh_opt_default = SshOptions::default();
    let ssh_opt = host.ssh_options.clone().unwrap_or_default();

    info!("Building SSH command for host {}", &name.green());
    let mut cmd = Command::new("ssh");

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    if let Some(connect_timeout) = ssh_opt.connect_timeout.or(ssh_opt_default.connect_timeout) {
        cmd.arg("-o")
            .arg(format!("ConnectTimeout={}", connect_timeout));
    }

    if let Some(server_alive_interval) = ssh_opt
        .server_alive_interval
        .or(ssh_opt_default.server_alive_interval)
    {
        cmd.arg("-o")
            .arg(format!("ServerAliveInterval={}", server_alive_interval));
    }

    cmd.arg("-N")
        .arg("-o")
        .arg("ControlPersist=yes")
        .arg("-o")
        .arg("BatchMode=yes");

    // set log path
    let log = TMP_DIR.join(format!("ant_{}.log", Local::now().to_rfc3339()));
    LOG_FILE.set(log.to_str().unwrap().to_string()).unwrap();
    cmd.arg("-E").arg(&log);

    if let Some(ref local_forward) = host.local_forward {
        for fwd in local_forward.iter() {
            info!(
                "Added local port forwarding from (R) {} to (L) {}",
                format_ip(&fwd.remote).green(),
                format_ip(&fwd.local).green(),
            );
            cmd.arg("-L").arg(format!("{}:{}", &fwd.local, &fwd.remote));
        }
    }

    if let Some(ref remote_forward) = host.remote_forward {
        for fwd in remote_forward.iter() {
            info!(
                "Added remote port forwarding from (L) {} to (R) {}",
                format_ip(&fwd.local).green(),
                format_ip(&fwd.remote).green(),
            );
            cmd.arg("-R").arg(format!("{}:{}", &fwd.remote, &fwd.local));
        }
    }

    if let Some(ref strict_host_key_checking) = ssh_opt
        .strict_host_key_checking
        .or(ssh_opt_default.strict_host_key_checking)
    {
        cmd.arg("-o");
        match strict_host_key_checking as _ {
            "accept-new" | "yes" | "no" => {
                cmd.arg(format!(
                    "StrictHostKeyChecking={}",
                    &strict_host_key_checking
                ));
            }
            _ => {
                warn!(
                    "Unknown option for StrictHostKeyChecking: {:?}",
                    &strict_host_key_checking
                );
                cmd.arg("StrictHostKeyChecking=yes");
            }
        };
    }

    if let Some(ref bind_address) = ssh_opt.bind_address.or(ssh_opt_default.bind_address) {
        cmd.arg("-b").arg(&bind_address);
    }

    if let Some(compression) = ssh_opt.compression.or(ssh_opt_default.compression) {
        cmd.arg("-o").arg(format!(
            "Compression={}",
            if compression { "yes" } else { "no" }
        ));
    }

    if let Some(ref ciphers) = ssh_opt.ciphers.or(ssh_opt_default.ciphers) {
        cmd.arg("-o").arg(format!("Ciphers={}", &ciphers));
    }

    if let Some(ref macs) = ssh_opt.macs.or(ssh_opt_default.macs) {
        cmd.arg("-o").arg(format!("Macs={}", &macs));
    }

    if let Some(exit_on_forward_failure) = ssh_opt
        .exit_on_forward_failure
        .or(ssh_opt_default.exit_on_forward_failure)
    {
        cmd.arg("-o").arg(format!(
            "ExitOnForwardFailure={}",
            if exit_on_forward_failure { "yes" } else { "no" }
        ));
    }

    cmd.arg("-o")
        .arg("IdentitiesOnly=yes")
        .arg("-i")
        .arg(&host.identity_file);

    // server connection part
    cmd.arg("-p")
        .arg(&host.port.unwrap_or(22).to_string())
        .arg("-l")
        .arg(&host.user)
        .arg(&host.hostname);

    debug!("{:#?}", &cmd);

    cmd
}
