use std::net::{UdpSocket,IpAddr};
use slog_scope::{info,error};
use chrono::SecondsFormat;
use crate::ntp;
use crate::response_strategy::ResponseStrategy;

pub struct Server {
    pub port: u16,
    pub addr: IpAddr,
    pub log_req_resp: bool,
    pub response_strategy: Box<dyn ResponseStrategy>,
}

impl Server {
    pub fn start_server(&mut self) -> std::io::Result<()> {
        let socket = UdpSocket::bind(self.addr.to_string() + ":" + &self.port.to_string())?;
        let mut buf = [0;65527];

        info!("started");

        loop { 
            match socket.recv_from(&mut buf) {
                Ok((amt, addr)) => {
                    //turns out ntp packets shorter than 48 bytes also valid? idk anymore
                    //im just going to assume that if the packet is shorter than the usual size the
                    //rest is filled with zeros
                    ntp::parser::parse_packet(&buf[0..(if amt > ntp::types::Packet::BASE_SIZE { amt } 
                                                       else { ntp::types::Packet::BASE_SIZE })]) 
                        .and_then(|packet| {
                            let packet = packet.1.unwrap();
                            info!("request from ip: {:}, size: {}, timestamp: {}, full_packet: {:?}", addr, amt, 
                                  packet.transit_timestamp.into_utc_datetime().to_rfc3339_opts(SecondsFormat::Nanos, true),
                                  packet);
                            let new_packet = self.response_strategy.process_packet(packet);
                            info!("responding to {:} with: ref: {}, org: {}, recv: {}, xmit: {}", addr,
                                new_packet.reference_timestamp.into_utc_datetime().to_rfc3339_opts(SecondsFormat::Nanos, true),
                                new_packet.origin_timestamp.into_utc_datetime().to_rfc3339_opts(SecondsFormat::Nanos, true),
                                new_packet.receive_timestamp.into_utc_datetime().to_rfc3339_opts(SecondsFormat::Nanos, true),
                                new_packet.transit_timestamp.into_utc_datetime().to_rfc3339_opts(SecondsFormat::Nanos, true));
                            let serialized = ntp::parser::serialize_packet(&new_packet);
                            if let Ok(buf) = serialized {
                                socket.send_to(&buf, addr).unwrap();
                            } else {
                                error!("serializing error: {:?} {:?}", serialized.err(), &buf);
                            } 
                            Ok(())
                        })
                        .map_err(|err| info!("parsing error: {} {:x?}", err, &buf[0..amt])).ok();
                },
                Err(err) => {
                    error!("error: {}", err);
                }
            }
        }
    }
}

