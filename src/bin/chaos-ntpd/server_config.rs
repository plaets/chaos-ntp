use serde::Deserialize;
use config::{Value};
use std::net::IpAddr;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug,Deserialize,Clone,)]
pub struct Server {
    pub address: IpAddr,
    pub port: u16,
    pub resp_strategy: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: IpAddr::from_str("127.0.0.1").unwrap(),
            port: 123,
            resp_strategy: "current_time".to_string(),
        }
    }
}

#[derive(Debug,Deserialize,Clone,Default)]
pub struct Log {
    pub log_all_requests: bool,
    pub log_request_data: bool,
}

#[derive(Debug,Deserialize,Clone,Default)]
pub struct ServerConfig {
    pub server: Server,
    pub log: Log,
    pub resp_strategy_conf: Vec<HashMap<String, HashMap<String, Value>>>,
}

