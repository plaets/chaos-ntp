use std::convert::{TryFrom,TryInto};
use nom::IResult;
use nom;
use byteorder::{BigEndian, WriteBytesExt};
use simple_error::SimpleError;
use super::types::*;

fn parse_header(input: (&[u8], usize)) -> IResult<(&[u8], usize), (u8, u8, u8)> {
    nom::error::context(
        "ntp_header",
        nom::sequence::tuple((
            nom::bits::complete::take(2usize),  //leap_indicator
            nom::bits::complete::take(3usize),  //version
            nom::bits::complete::take(3usize),  //mode
        ))
    )(input)
}

fn parse_metadata(input: &[u8]) -> IResult<&[u8], (u8, i8, i8, Short, Short, &[u8])> {
    nom::error::context(
        "ntp_metadata",
        nom::sequence::tuple((
            nom::number::complete::u8,                                  //stratum
            nom::number::complete::i8,                                  //poll
            nom::number::complete::i8,                                  //precision
            nom::number::complete::u32(nom::number::Endianness::Big),   //root_delay
            nom::number::complete::u32(nom::number::Endianness::Big),   //root_dispersion
            nom::bytes::complete::take(4usize),                         //reference_id
        ))
    )(input)
}

fn parse_timedata(input: &[u8]) -> IResult<&[u8], (Timestamp, Timestamp, Timestamp, Timestamp)> {
    nom::error::context(
        "ntp_timedata",
        nom::sequence::tuple((
            nom::number::complete::u64(nom::number::Endianness::Big),   //reference_timestamp
            nom::number::complete::u64(nom::number::Endianness::Big),   //origin_timestamp
            nom::number::complete::u64(nom::number::Endianness::Big),   //reference_timestamp
            nom::number::complete::u64(nom::number::Endianness::Big),   //transit_timestamp
        ))
    )(input)
}

fn parse_auth(input: &[u8]) -> IResult<&[u8], Option<Auth>> {
    nom::error::context(
        "ntp_auth",
        nom::combinator::map(
            nom::combinator::opt(
                nom::sequence::tuple((
                    nom::number::complete::u32(nom::number::Endianness::Big),   //key_indentifier
                    nom::number::complete::u128(nom::number::Endianness::Big),  //digest
                ))
            ),
            |auth| auth.and_then(|a| Some(Auth { key_indentifier: a.0, digest: a.1 }))
        )
    )(input)
}

pub fn parse_packet(input: &[u8]) -> IResult<(&[u8], usize), Result<Packet, SimpleError>> {
    nom::error::context(
        "ntp_packet",
        nom::combinator::map(
            nom::sequence::tuple((
                parse_header,
                nom::bits::bytes::<_,_,nom::error::Error<_>,_,_>(
                    nom::sequence::tuple((
                        parse_metadata,
                        parse_timedata,
                        parse_auth,
                    ))
                ),
            )),
            |(
              (leap_indicator, version, mode),
              ((stratum, poll, precision, root_delay, root_dispersion, reference_id),
               (reference_timestamp, origin_timestamp, receive_timestamp, transit_timestamp),
               auth),
            )| {
                Ok(Packet {
                    version,
                    leap_indicator: LeapIndicator::try_from(leap_indicator).map_err(|_| "invalid leap_indicator")?,
                    mode: Mode::try_from(mode).map_err(|_| "invalid mode")?,

                    stratum: stratum.try_into()?,
                    poll, precision, root_delay, root_dispersion,
                    reference_id: reference_id[0..4].try_into().map_err(|_| "invlaid reference_id")?,

                    reference_timestamp, origin_timestamp, 
                    receive_timestamp, transit_timestamp,

                    auth,
                })
            },
        )
    )((input,0))
}

pub fn serialize_packet(packet: &Packet) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut data: Vec<u8> = Vec::with_capacity(packet.size());

    let packet_indicator: u8 = packet.leap_indicator.clone().into();
    let mode: u8 = packet.mode.into();
    let header = (packet_indicator << 6) | (packet.version << 3) | (mode);
    data.write_u8(header)?;
    data.write_u8(packet.stratum.try_into()?)?;
    data.write_i8(packet.poll)?;
    data.write_i8(packet.precision)?;
    data.write_u32::<BigEndian>(packet.root_delay)?;
    data.write_u32::<BigEndian>(packet.root_dispersion)?;
    for n in &packet.reference_id { data.write_u8(*n)?; }
    data.write_u64::<BigEndian>(packet.reference_timestamp)?;
    data.write_u64::<BigEndian>(packet.origin_timestamp)?;
    data.write_u64::<BigEndian>(packet.receive_timestamp)?;
    data.write_u64::<BigEndian>(packet.transit_timestamp)?;

    //TODO: EXTENSIONS!!!
    
    if let Some(auth) = &packet.auth { 
        data.write_u32::<BigEndian>(auth.key_indentifier)?;
        data.write_u128::<BigEndian>(auth.digest)?;
    }

    Ok(data)
}

