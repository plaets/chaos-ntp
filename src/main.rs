use std::net::UdpSocket;
use rand::Rng;
use rand::random;
mod ntp;
use ntp::{TimestampTrait,Short};

struct Server {
    time: ntp::Timestamp,
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

    fn process_packet(&self, mut packet: ntp::Packet) -> ntp::Packet {
        let rand_time = random::<u64>();

        packet.leap_indicator = ntp::LeapIndicator::NoWarning;
        packet.version = 4;
        packet.mode = ntp::Mode::Server;
        packet.stratum = ntp::Stratum::SecondaryServer(5);
        packet.poll = 6;
        packet.precision = -14;
        packet.root_delay = (0 as Short).set_fraction(10);
        packet.root_dispersion = (0 as Short).set_fraction(10);
        packet.reference_id = [0,0,0,0];
        packet.origin_timestamp = packet.transit_timestamp;
        packet.reference_timestamp = rand_time.set_seconds(rand_time.get_seconds() - 10); //last set
        packet.receive_timestamp = rand_time;
        packet.transit_timestamp = rand_time.set_seconds(rand_time.get_seconds() - 1);
        packet.auth = None;
        packet
    }
}


fn main() -> std::io::Result<()> {
    let port: u16 = 123;
    let ip = String::from("0.0.0.0");
    let mut server = Server::new();

    let socket = UdpSocket::bind(ip + ":" + &port.to_string())?;
    let mut buf = [0;65527];

    //this block of code is awful and i absolutely fucking hate it
    loop { 
        match socket.recv_from(&mut buf) {
            Ok((amt, addr)) => {
                println!("{:}, {}", addr, amt);
                println!("{:x?}", &buf[0..amt]);
                //turns out shorter ntp packets are also valid? idfk know anymore
                //ntp::parse_packet(&buf[0..amt])
                ntp::parse_packet(&buf)
                    .and_then(|packet| {
                        let mut packet = packet.1.unwrap();
                        println!("{:?}", &packet);
                        let new_packet = server.process_packet(packet);
                        println!("responding with: {:?}", &new_packet);
                        let serialized = ntp::serialize_packet(&new_packet);
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

