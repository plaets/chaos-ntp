use std::net::UdpSocket;
use rand::random;
mod ntp;
use ntp::types::{TimestampTrait,Short};

struct Server {
    time: ntp::types::Timestamp,
}

impl Server {
    fn new() -> Self { 
        Self {
            time: random::<u64>(),
        }
    }

    fn get_time(&self) -> u64 {
        self.time
    }

    fn process_packet(&self, packet: ntp::types::Packet) -> ntp::types::Packet {
        let rand_time = random::<u64>();

        ntp::types::Packet {
            leap_indicator: ntp::types::LeapIndicator::NoWarning,
            version: 4,
            mode: ntp::types::Mode::Server,
            stratum: ntp::types::Stratum::SecondaryServer(4),
            poll: 6,
            precision: -16,
            root_delay: (0 as Short).set_fraction(1000),
            root_dispersion: (0 as Short).set_fraction(1000),
            reference_id: [0,0,0,0],
            origin_timestamp: packet.transit_timestamp,
            reference_timestamp: rand_time.set_seconds(rand_time.get_seconds() - 10), //last set
            receive_timestamp: rand_time,
            transit_timestamp: rand_time.set_seconds(rand_time.get_seconds() - 1),
            auth: None,
        }
    }
}


fn main() -> std::io::Result<()> {
    let port: u16 = 123;
    let ip = String::from("0.0.0.0");
    let mut server = Server::new();

    let socket = UdpSocket::bind(ip + ":" + &port.to_string())?;
    let mut buf = [0;65527];

    loop { 
        match socket.recv_from(&mut buf) {
            Ok((amt, addr)) => {
                println!("{:}, {}", addr, amt);
                println!("{:x?}", &buf[0..amt]);
                //turns out ntp packets shorter than the udp payload length are also valid? idk know anymore
                //ntp::parse_packet(&buf[0..amt])
                ntp::parser::parse_packet(&buf[0..(if amt > 76 { amt } else { 76 })])
                    .and_then(|packet| {
                        let packet = packet.1.unwrap();
                        println!("{:?}", &packet);
                        let new_packet = server.process_packet(packet);
                        println!("responding with: {:?}", &new_packet);
                        let serialized = ntp::parser::serialize_packet(&new_packet);
                        if let Ok(buf) = serialized {
                            println!("{:?}", &buf);
                            socket.send_to(&buf, addr).unwrap();
                        } else {
                            println!("serializing error: {:?}", serialized.err());
                        }
                        Ok(())
                    })
                    .map_err(|err| println!("parsing error: {}", err)).unwrap();
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}

