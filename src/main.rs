use core::ops::Range;
use std::net::UdpSocket;
use std::time::{SystemTime,Instant};
use rand::{random,distributions::{Distribution,Uniform}};
mod ntp;
use ntp::types::{TimestampTrait,Short};

struct Server {
    time_offset: i64, //time offset in seconds
    last_update: Instant,
    offset_range: Range<i64>,
    counter: u32,
}

impl Server {
    fn new() -> Self { 
        let offset_range = (0)..(60*60*24*2);
        Self {
            //time_offset: Uniform::from((-60*60*24*180)..(60*60*24*365*1)).sample(&mut rand::thread_rng()),
            offset_range: offset_range.clone(),
            //time_offset: Uniform::from(offset_range).sample(&mut rand::thread_rng()),
            time_offset: 0,
            last_update: Instant::now(),
            counter: 0,
        }
    }

    //TODO: this magic 70*365... represents the offset of unix epoch in relation to ntp epoch
    //(1.1.1900). sorta represents because the date is still wrong by a few days when time offset is 0
    //thats where the seconds magic number comes in (17 days)
    //now that i think about it its probably because of leap years...
    fn get_time(&mut self) -> u64 {
        self.time_offset += 1;
        self.counter += 1;

        if self.time_offset > 0 {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + (70*365*24*60*60) + (17*60*60*24) + (self.time_offset as u64)
        } else {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + (70*365*24*60*60) + (17*60*60*24) - (self.time_offset.abs() as u64)
        }
    }

    fn process_packet(&mut self, packet: ntp::types::Packet) -> ntp::types::Packet {
        let rand_time = (ntp::types::Timestamp::from(0)).set_seconds(self.get_time() as u32); 
        let fraction = 0;//rand::random::<u32>();

        ntp::types::Packet {
            leap_indicator: ntp::types::LeapIndicator::NoWarning,
            version: 4,
            mode: ntp::types::Mode::Server,
            stratum: ntp::types::Stratum::SecondaryServer(4),
            poll: 6,
            precision: -16,
            root_delay: Short::from(0).set_fraction(1000),
            root_dispersion: Short::from(0).set_fraction(1000),
            reference_id: [0,0,0,0],
            origin_timestamp: packet.transit_timestamp,
            reference_timestamp: rand_time.set_seconds(rand_time.get_seconds()).set_fraction(fraction), //last set
            receive_timestamp: rand_time.set_fraction(fraction),
            transit_timestamp: rand_time.set_seconds(rand_time.get_seconds()).set_fraction(fraction),
            auth: None,
        }
    }
}


fn main() -> std::io::Result<()> {
    let port: u16 = 123;
    let ip = String::from("0.0.0.0");
    let socket = UdpSocket::bind(ip + ":" + &port.to_string())?;

    let mut server = Server::new();
    let mut buf = [0;65527];

    println!("started");

    loop { 
        match socket.recv_from(&mut buf) {
            Ok((amt, addr)) => {
                println!("{:}, {}", addr, amt);
                println!("{:x?}", &buf[0..amt]);
                //turns out ntp packets shorter than 48 bytes also valid? idk anymore
                //im just going to assume that if the packet is shorter than the usual size the
                //rest is filled with zeros
                ntp::parser::parse_packet(&buf[0..(if amt > ntp::types::Packet::BASE_SIZE { amt } 
                                                   else { ntp::types::Packet::BASE_SIZE })]) 
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
                    .map_err(|err| println!("parsing error: {}", err)).ok();
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}

