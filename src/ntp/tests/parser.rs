use crate::ntp::types::*;
use crate::ntp::parser::*;
use crate::ntp::constants::*;

//TODO: timestamp parsing/formatting
//TODO: fractions
//TODO: invalid packets
//TODO: short packets

#[test]
fn valid_server_packet() {
    static PACKET: &'static [u8] = &[
        0x24,                                           //no leap warning, ntpv4, server
        0x02,                                           //stratum 2
        0x03,                                           //poll interval 3 (invalid?)
        0xe8,                                           //clock precision 0 seconds (?)
        0x00, 0x00, 0x0f, 0x8e,                         //root delay 0,060760
        0x00, 0x00, 0x05, 0x4a,                         //root dispersion 0,0260660 seconds
        0x84, 0xa3, 0x60, 0x04,                         //reference id time-d-b.nist.gov 
        0xe3, 0x8c, 0x4e, 0xf1, 0x3f, 0x42, 0x7c, 0xcc, //reference timestamp (Dec 22, 2020 10:54:41.247108268 UTC)
        0xe3, 0x8c, 0x4f, 0xd4, 0xd7, 0x47, 0x2d, 0xcd, //origin timestamp (Dec 22, 2020 10:58:28.840929853 UTC)
        0xe3, 0x8c, 0x4f, 0xd4, 0xe9, 0xb0, 0xee, 0x14, //receive timestamp (Dec 22, 2020 10:58:28.912855987 UTC)
        0xe3, 0x8c, 0x4f, 0xd4, 0xe9, 0xb1, 0xd8, 0x45  //transit timestamp (Dec 22, 2020 10:58:28.912869946 UTC)
    ];

    let parsed = parse_packet(PACKET).unwrap().1.unwrap();

    assert_eq!(parsed.leap_indicator, LeapIndicator::NoWarning);
    assert_eq!(parsed.version, 4);
    assert_eq!(parsed.mode, Mode::Server);
    assert_eq!(parsed.stratum, Stratum::SecondaryServer(2));
    assert_eq!(parsed.poll, 3);
    assert_eq!(parsed.precision, -24); //TODO
    assert_eq!(parsed.root_delay, Short::from(0).set_seconds(0).set_fraction(0x0f8e)); //TODO: how to interpret fractions?
    assert_eq!(parsed.root_dispersion, Short::from(0).set_seconds(0).set_fraction(0x054a)); 
    assert_eq!(parsed.reference_id, [0x84, 0xa3, 0x60, 0x04]);
    assert_eq!(parsed.reference_timestamp, Timestamp::from(0).set_seconds(0xe38c4ef1).set_fraction(0x3f427ccc)); 
    assert_eq!(parsed.origin_timestamp, Timestamp::from(0).set_seconds(0xe38c4fd4).set_fraction(0xd7472dcd)); 
    assert_eq!(parsed.receive_timestamp, Timestamp::from(0).set_seconds(0xe38c4fd4).set_fraction(0xe9b0ee14)); 
    assert_eq!(parsed.transit_timestamp, Timestamp::from(0).set_seconds(0xe38c4fd4).set_fraction(0xe9b1d845)); 

    assert_eq!(parsed.reference_timestamp.into_utc_datetime().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
        "2020-12-22T10:54:41.247108268Z");
    assert_eq!(parsed.origin_timestamp.into_utc_datetime().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
        "2020-12-22T10:58:28.840929853Z");
    assert_eq!(parsed.receive_timestamp.into_utc_datetime().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
        "2020-12-22T10:58:28.912855987Z");
    assert_eq!(parsed.transit_timestamp.into_utc_datetime().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
        "2020-12-22T10:58:28.912869946Z");

    assert_eq!(serialize_packet(&parsed).unwrap(), PACKET);
}

#[test]
fn valid_client_packet() {
    static PACKET: &'static [u8] = &[
        0xe3,                                           //unknown leap, ntpv4, client
        0x00,                                           //stratum unspecified
        0x03,                                           //poll interval 3 (interval?)
        0xfa,                                           //clock precision (0,015625)
        0x00, 0x01, 0x00, 0x00,                         //root delay 1 second
        0x00, 0x01, 0x00, 0x00,                         //root dispersion 1 second
        0x00, 0x00, 0x00, 0x00,                         //reference id
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //reference timestamp
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //origin timestamp
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //receive timestamp
        0xe3, 0x8c, 0x4f, 0xd4, 0xd7, 0x47, 0x2d, 0xcd  //transit timestamp (Dec 22, 2020 10:58:28.840929853 UTC)
    ];

    let parsed = parse_packet(PACKET).unwrap().1.unwrap();

    assert_eq!(parsed.leap_indicator, LeapIndicator::Unknown);
    assert_eq!(parsed.version, 4);
    assert_eq!(parsed.mode, Mode::Client);
    assert_eq!(parsed.stratum, Stratum::Unspecified);
    assert_eq!(parsed.poll, 3);
    assert_eq!(parsed.precision, -6); //TODO
    assert_eq!(parsed.root_delay, Short::from(0).set_seconds(1).set_fraction(0)); //TODO: how to interpret fractions?
    assert_eq!(parsed.root_dispersion, Short::from(0).set_seconds(1).set_fraction(0)); 
    assert_eq!(parsed.reference_id, [0x0, 0x0, 0x0, 0x0]);
    assert_eq!(parsed.reference_timestamp, Timestamp::from(0)); 
    assert_eq!(parsed.origin_timestamp, Timestamp::from(0)); 
    assert_eq!(parsed.receive_timestamp, Timestamp::from(0)); 
    assert_eq!(parsed.transit_timestamp, Timestamp::from(0).set_seconds(0xe38c4fd4).set_fraction(0xd7472dcd)); 

    assert_eq!(serialize_packet(&parsed).unwrap(), PACKET);
}

//TODO: this packet was made up, catch one from real traffic
//TODO: digest verification
//TODO: padding?
#[test]
fn valid_client_packet_with_extensions() {
    static PACKET: &'static [u8] = &[
        0xe3,                                           //unknown leap, ntpv4, client
        0x00,                                           //stratum unspecified
        0x03,                                           //poll interval 3 (interval?)
        0xfa,                                           //clock precision (0,015625)
        0x00, 0x01, 0x00, 0x00,                         //root delay 1 second
        0x00, 0x01, 0x00, 0x00,                         //root dispersion 1 second
        0x00, 0x00, 0x00, 0x00,                         //reference id
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //reference timestamp
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //origin timestamp
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //receive timestamp
        0xe3, 0x8c, 0x4f, 0xd4, 0xd7, 0x47, 0x2d, 0xcd, //transit timestamp (Dec 22, 2020 10:58:28.840929853 UTC)
        0x00, 0x20, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, //noop extension field
        0x00, 0x00, 0x00, 0x00,                         //auth key_id
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //auth digest
    ];

    let parsed = parse_packet(PACKET).unwrap().1.unwrap();

    assert_eq!(parsed.leap_indicator, LeapIndicator::Unknown);
    assert_eq!(parsed.version, 4);
    assert_eq!(parsed.mode, Mode::Client);
    assert_eq!(parsed.stratum, Stratum::Unspecified);
    assert_eq!(parsed.poll, 3);
    assert_eq!(parsed.precision, -6); //TODO
    assert_eq!(parsed.root_delay, Short::from(0).set_seconds(1).set_fraction(0)); //TODO: how to interpret fractions?
    assert_eq!(parsed.root_dispersion, Short::from(0).set_seconds(1).set_fraction(0)); 
    assert_eq!(parsed.reference_id, [0x0, 0x0, 0x0, 0x0]);
    assert_eq!(parsed.reference_timestamp, Timestamp::from(0)); 
    assert_eq!(parsed.origin_timestamp, Timestamp::from(0)); 
    assert_eq!(parsed.receive_timestamp, Timestamp::from(0)); 
    assert_eq!(parsed.transit_timestamp, Timestamp::from(0).set_seconds(0xe38c4fd4).set_fraction(0xd7472dcd)); 
    assert_eq!(parsed.extensions.clone().unwrap()[0].field_type, ExtensionFieldType::NOOP);
    assert_eq!(parsed.auth.unwrap().key_indentifier, 0);
    assert_eq!(parsed.auth.unwrap().digest, 0);

    assert_eq!(serialize_packet(&parsed).unwrap(), PACKET);
}

//packet with extension field with size zero
#[test]
fn invalid_packet_ext_field_length_zero() {
    static PACKET: &'static [u8] = &[
        23, 0, 3, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    let parsed = parse_packet(PACKET);

    match parsed.err().unwrap() {
        nom::Err::Error(err) => assert_eq!(err.code, nom::error::ErrorKind::Verify),
        _ => assert!(false),
    }
}

