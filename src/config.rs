use config::{Config,ConfigError};
use std::net::IpAddr;
use std::collections::HashMap;

#[derive(Debug,Deserialize,Clone)]
pub struct Server {
    pub address: IpAddr,
    pub port: u16,
    pub resp_strategy: String,
}

#[derive(Debug,Deserialize,Clone)]
pub struct Log {
    pub log_all_requests: bool,
    pub log_request_data: bool,
}

#[derive(Debug,Deserialize,Clone)]
pub struct Config {
    pub server: Server,
    pub log: Log,
    pub resp_strategy_conf: Vec<HashMap<String, String>>,
}

