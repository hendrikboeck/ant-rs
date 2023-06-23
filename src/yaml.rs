use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Root {
    pub version: String,
    pub hosts: HashMap<String, Host>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Host {
    pub hostname: String,
    pub port: Option<u16>,
    pub identity_file: String,
    pub user: String,
    pub local_forward: Option<Vec<Forward>>,
    pub remote_forward: Option<Vec<Forward>>,
    pub ssh_options: Option<SshOptions>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Forward {
    pub local: String,
    pub remote: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SshOptions {
    pub strict_host_key_checking: Option<String>,
    pub bind_address: Option<String>,
    pub compression: Option<bool>,
    pub ciphers: Option<String>,
    pub connect_timeout: Option<usize>,
    pub server_alive_interval: Option<usize>,
    pub macs: Option<String>,
    pub exit_on_forward_failure: Option<bool>,
}

impl Default for SshOptions {
    fn default() -> Self {
        Self {
            strict_host_key_checking: Some("yes".into()),
            bind_address: None,
            compression: Some(false),
            ciphers: None,
            connect_timeout: Some(10),
            server_alive_interval: Some(600),
            macs: None,
            exit_on_forward_failure: Some(true),
        }
    }
}
