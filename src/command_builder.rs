use colored::Colorize;
use log::{debug, info, warn};
use once_cell::sync::OnceCell;
use std::process::Stdio;
use tokio::process::Command;

use chrono::Local;

use crate::configuration::TMP_DIR;
use crate::yaml::{Host, SshOptions};

pub static LOG_FILE: OnceCell<String> = OnceCell::new();

trait ToSshAddr {
    fn to_ssh_addr(&self) -> Self;
}

impl ToSshAddr for String {
    fn to_ssh_addr(&self) -> Self {
        if self.contains(':') {
            self.clone()
        } else {
            format!("localhost:{self}")
        }
    }
}

pub fn build_ssh_process(name: &str, host: &Host) -> Command {
    let ssh_opt_default = SshOptions::default();
    let ssh_opt = host.ssh_options.as_ref().unwrap_or(&ssh_opt_default);

    info!("Building SSH command for host {}", &name.green());
    let mut cmd = Command::new("ssh");

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    cmd.arg("-N")
        .arg("-o")
        .arg("ControlPersist=yes")
        .arg("-o")
        .arg("BatchMode=yes");

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

    // set log path
    let log = TMP_DIR.join(format!("ant_{}.log", Local::now().to_rfc3339()));
    LOG_FILE.set(log.to_str().unwrap().to_string()).unwrap();
    cmd.arg("-E").arg(&log);

    if let Some(ref local_forward) = host.local_forward {
        for fwd in local_forward.iter() {
            let remote = fwd.remote.to_ssh_addr();
            let local = fwd.local.to_ssh_addr();

            info!(
                "Added local port forwarding from (R) {} to (L) {}",
                &remote.green(),
                &local.green(),
            );
            cmd.arg("-L").arg(format!("{}:{}", &local, &remote));
        }
    }

    if let Some(ref remote_forward) = host.remote_forward {
        for fwd in remote_forward.iter() {
            let remote = fwd.remote.to_ssh_addr();
            let local = fwd.local.to_ssh_addr();

            info!(
                "Added remote port forwarding from (L) {} to (R) {}",
                &local.green(),
                &remote.green(),
            );
            cmd.arg("-R").arg(format!("{}:{}", &remote, &local));
        }
    }

    if let Some(strict_host_key_checking) = ssh_opt
        .strict_host_key_checking
        .as_ref()
        .or(ssh_opt_default.strict_host_key_checking.as_ref())
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

    if let Some(bind_address) = ssh_opt
        .bind_address
        .as_ref()
        .or(ssh_opt_default.bind_address.as_ref())
    {
        cmd.arg("-b").arg(&bind_address);
    }

    if let Some(compression) = ssh_opt.compression.or(ssh_opt_default.compression) {
        cmd.arg("-o").arg(format!(
            "Compression={}",
            if compression { "yes" } else { "no" }
        ));
    }

    if let Some(ciphers) = ssh_opt
        .ciphers
        .as_ref()
        .or(ssh_opt_default.ciphers.as_ref())
    {
        cmd.arg("-o").arg(format!("Ciphers={}", &ciphers));
    }

    if let Some(macs) = ssh_opt.macs.as_ref().or(ssh_opt_default.macs.as_ref()) {
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
