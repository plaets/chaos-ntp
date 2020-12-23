use std::net::{IpAddr};
use std::str::FromStr;
mod ntp;
mod logger;
use logger::setup_logger;
mod server;
mod response_strategy;

fn main() -> std::io::Result<()> {
    let _guard = setup_logger();
    let mut server = server::Server {
        port: 123,
        addr: IpAddr::from_str("0.0.0.0").map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
        log_req_resp: true,
        response_strategy: Box::from(response_strategy::CurrentTime{}),
    };
    server.start_server()
}

