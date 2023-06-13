use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Root {
    pub version: String,
    pub hosts: HashMap<String, Host>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Host {
    pub hostname: String,
    pub port: u16,
    pub identity_file: String,
    pub user: String,
    pub local_forward: Vec<Forward>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Forward {
    pub local: u16,
    pub remote: String,
}
