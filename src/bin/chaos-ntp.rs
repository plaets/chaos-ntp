use std::net::{IpAddr};
use std::str::FromStr;
use config::{Config,File};
use chaos_ntp::server;
use chaos_ntp::response_strategy::{ResponseStrategyCtor};
use chaos_ntp::logger::setup_logger;
use chaos_ntp::config::ServerConfig;

fn main() -> std::io::Result<()> {
    let _guard = setup_logger();

    let config: ServerConfig = Config::new()
        .merge(File::with_name("config.cfg")).unwrap().clone().try_into::<ServerConfig>().unwrap().clone();

    let rs = inventory::iter::<&dyn ResponseStrategyCtor>.into_iter().find(|s| s.name() == config.server.resp_strategy).unwrap().new();
    let mut server = server::Server {
        port: config.server.port,
        addr: config.server.address,
        log_req_resp: config.log.log_all_requests,
        response_strategy: rs,
    };
    server.start_server()
}

