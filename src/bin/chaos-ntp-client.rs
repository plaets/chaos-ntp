use std::net::{UdpSocket, ToSocketAddrs};
use clap::{Arg, App};
use chrono::Utc;
use chaos_ntp::ntp::{parser::{serialize_packet, parse_packet}, types::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = App::new("chaos-ntp client")
        .version("1.0")
        .author("plates <plates.jsnm@gmail.com>")
        .about("ntp client")
        .arg(Arg::with_name("ADDR")
             .help("ntp server address")
             .required(true)
             .index(1))
        .arg(Arg::with_name("port")
             .help("ntp server port")
             .short("p")
             .required(false))
        .arg(Arg::with_name("verbose")
             .help("print more info")
             .short("v")
             .long("verbose")
             .required(false))
        .arg(Arg::with_name("raw-timestamps")
             .help("print timestamps as numbers")
             .long("raw-timestamps")
             .required(false))
        .get_matches();

    let addr = args.value_of("ADDR").unwrap();
    let port = args.value_of("port").unwrap_or("123");
    let verbose = args.is_present("verbose");
    let raw_timestamps = args.is_present("raw-timestamps");

    let resolved_addr = (String::from(addr) + ":" + port)
                   .to_socket_addrs()?
                   .collect::<Vec<_>>();
    
    let packet = Packet {
        leap_indicator: LeapIndicator::Unknown,
        version: 4,
        mode: Mode::Client,
        stratum: Stratum::Unsynchronized,
        poll: 4,
        precision: -6,
        root_delay: 65535.into(),
        root_dispersion: 65535.into(),
        reference_id: *b"INIT",
        reference_timestamp: Timestamp(0),
        origin_timestamp: Timestamp(0),
        receive_timestamp: Timestamp(0),
        transit_timestamp: Timestamp::from_utc_datetime(Utc::now()).unwrap(),
        extensions: None,
        auth: None
    };

    let packet_data = serialize_packet(&packet).unwrap();
    let mut response_buf = [0; Packet::MAX_SIZE];

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.send_to(&packet_data, resolved_addr.first().unwrap())?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(10)))?;

    let (size, from) = socket.recv_from(&mut response_buf)?;
    let response_packet = parse_packet(&response_buf[..size]).unwrap().1.unwrap();

    println!("response from: {:?}", from);

    if verbose {
        println!("response size: {:?}", size);
        println!("leap indicator: {:?}", response_packet.leap_indicator);
        println!("version: {:?}", response_packet.version);
        println!("mode: {:?}", response_packet.mode);
        println!("stratum: {:?}", response_packet.stratum);
        println!("poll: {:?}", response_packet.poll);
        println!("precision: {:?}", response_packet.precision);

        if raw_timestamps {
            println!("root delay: {:?}", response_packet.root_delay);
            println!("root dispersion: {:?}", response_packet.root_dispersion);
        } else {
            println!("root delay: {:?}", response_packet.root_delay.into_duration());
            println!("root dispersion: {:?}", response_packet.root_dispersion.into_duration());
        }

        println!("reference id: {:?}", response_packet.reference_id);
        println!("extensions: {:?}", response_packet.extensions);
        println!("auth: {:?}", response_packet.auth);
    }

    if raw_timestamps {
        println!("reference timestamp: {:?}", response_packet.reference_timestamp);
        println!("origin timestamp: {:?}", response_packet.origin_timestamp);
        println!("receive timestamp: {:?}", response_packet.receive_timestamp);
        println!("transit timestamp: {:?}", response_packet.transit_timestamp);
    } else {
        println!("reference timestamp: {:?}", response_packet.reference_timestamp.into_utc_datetime());
        println!("origin timestamp: {:?}", response_packet.origin_timestamp.into_utc_datetime());
        println!("receive timestamp: {:?}", response_packet.receive_timestamp.into_utc_datetime());
        println!("transit timestamp: {:?}", response_packet.transit_timestamp.into_utc_datetime());
    }

    Ok(())
}
