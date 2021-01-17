use std::net::IpAddr;
use std::collections::HashMap;
use std::str::FromStr;
use serde::{Deserialize,Serialize};
use toml::value::Value;
use slog::Level;

#[derive(Debug,Serialize,Deserialize,Clone,)]
pub struct Server {
    pub address: IpAddr,
    pub port: u16,
    pub resp_strategy: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: IpAddr::from_str("0.0.0.0").unwrap(),
            port: 123,
            resp_strategy: "current_time".to_string(),
        }
    }
}

#[derive(Serialize,Deserialize)]
#[serde(remote = "Level")]
#[serde(rename_all = "lowercase")]
enum LevelDef {
    Trace,
    Debug,
    Info,
    Error,
    Warning,
    Critical,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Log {
    pub log_all_requests: bool, //log requests
    #[serde(with = "LevelDef")]
    pub level: Level,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            log_all_requests: false,
            level: Level::Info,
        }
    }
}

#[derive(Debug,Serialize,Deserialize,Clone,Default)]
pub struct ServerConfig {
    pub server: Server,
    pub log: Log,
    pub resp_strategy_conf: HashMap<String, HashMap<String, Value>>,
}

