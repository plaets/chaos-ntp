use config::{Config,File};
mod server;
mod response_strategy;
use response_strategy::{ResponseStrategyCtor};
mod logger;
use logger::setup_logger;
mod server_config;
use server_config::ServerConfig;

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

