use config::{Config,File};
mod server;
mod response_strategy;
use response_strategy::{ResponseStrategyCtor};
mod logger;
use logger::setup_logger;
mod server_config;
use server_config::ServerConfig;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> std::io::Result<()> {
    let mut config_rep: Config = Config::new();
    config_rep.merge(File::with_name("config.cfg")).unwrap().clone();
    let config = config_rep.try_into::<ServerConfig>().unwrap().clone();

    let _guard = setup_logger(config.log.level);

    let rs = inventory::iter::<&dyn ResponseStrategyCtor>.into_iter().find(|s| s.name() == config.server.resp_strategy).unwrap().new();

    let mut server = server::Server {
        port: config.server.port,
        addr: config.server.address,
        log_all_requests: config.log.log_all_requests,
        response_strategy: rs,
    };
    server.start_server()
}

