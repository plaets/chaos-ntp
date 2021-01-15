use std::net::{IpAddr};
use std::str::FromStr;
use chaos_ntp::server;
use chaos_ntp::response_strategy::{ResponseStrategyCtor};
use chaos_ntp::logger::setup_logger;

fn main() -> std::io::Result<()> {
    let _guard = setup_logger();
    let rs = inventory::iter::<&dyn ResponseStrategyCtor>.into_iter().find(|s| s.name() == "single_offset").unwrap().new();
    let mut server = server::Server {
        port: 123,
        addr: IpAddr::from_str("0.0.0.0").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
        log_req_resp: true,
        response_strategy: rs,
    };
    server.start_server()
}

