use std::convert::{TryFrom,TryInto};
use nom::IResult;
use nom;
use num_enum::{IntoPrimitive,TryFromPrimitive};
use byteorder::{BigEndian, WriteBytesExt};
use simple_error::SimpleError;

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
pub enum KoD {
    ACST,
    AUTH,
    AUTO,
    BCST,
    CRYP,
    DENY,
    DROP,
    RSTR,
    INIT,
    MCST,
    NKEY,
    RATE,
    RMOT,
    STEP,
    Unknown([u8;4]),
}

macro_rules! into_array {
    ($x:expr, $len:expr) => {
        { $x.as_bytes()[..($len)].try_into().unwrap()}
    }
}

impl Into<[u8;4]> for KoD {
    fn into(self) -> [u8;4] {
        match self {
            Self::ACST => into_array!("ACST", 4),
            Self::AUTH => into_array!("AUTH", 4),
            Self::AUTO => into_array!("AUTO", 4),
            Self::BCST => into_array!("BCST", 4),
            Self::CRYP => into_array!("CRYP", 4),
            Self::DENY => into_array!("DENY", 4),
            Self::DROP => into_array!("DROP", 4),
            Self::RSTR => into_array!("RSTR", 4),
            Self::INIT => into_array!("INIT", 4),
            Self::MCST => into_array!("MCST", 4),
            Self::NKEY => into_array!("NKEY", 4),
            Self::RATE => into_array!("RATE", 4),
            Self::RMOT => into_array!("RMOT", 4),
            Self::STEP => into_array!("STEP", 4),
            Self::Unknown(data) => data
        }
    }
}

#[derive(Debug,Eq,PartialEq,Clone,Copy,IntoPrimitive,TryFromPrimitive)]
#[repr(u8)]
pub enum LeapIndicator {
    NoWarning = 0,
    LastMinute61Seconds = 1,
    LastMinute59Seconds = 2,
    Unknown = 3,
}

#[derive(Debug,Clone,Copy)]
pub enum Stratum {
    Unspecified,
    PrimaryServer,
    SecondaryServer(u8),
    Unsynchronized,
    Reserved(u8),
}

impl TryFrom<u8> for Stratum {
    type Error = SimpleError;

    fn try_from(value: u8) -> Result<Stratum, Self::Error> {
        Ok(match value {
            0 => Stratum::Unspecified,
            1 => Stratum::PrimaryServer,
            2..=15 => Stratum::SecondaryServer(value),
            16 => Stratum::Unsynchronized,
            17..=255 => Stratum::Reserved(value),
        })
    }
}


impl TryInto<u8> for Stratum {
    type Error = SimpleError;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Stratum::Unspecified => Ok(0),
            Stratum::PrimaryServer => Ok(1),
            Stratum::SecondaryServer(v) => if (2..=15).contains(&v) { Ok(v) } 
                else { Err(SimpleError::new("invalid SecondaryServer value")) },
            Stratum::Unsynchronized => Ok(16),
            Stratum::Reserved(v) => if (17..=255).contains(&v) { Ok(v) } 
                else { Err(SimpleError::new("invalid Reserved value")) },
        }
    }
}

#[derive(Debug,Eq,PartialEq,Clone,Copy,IntoPrimitive,TryFromPrimitive)]
#[repr(u8)]
pub enum Mode {
    Reserved = 0,
    SymmetricActive = 1,
    SymmetricPassive = 2,
    Client = 3,
    Server = 4,
    Broadcast = 5,
    NTPControlMessage = 6,
    ReservedForPrivate = 7,
}

#[derive(Debug,Clone)]
pub struct Auth {
    key_indentifier: u32,   //32 bits, optional
    digest: u128,           //128 bits, optional
}

#[derive(Debug)]
pub struct Date {
    era_number: u32,
    era_offset: u32,
    fraction: u32,
}
//what even is this
//do i need this
//rfc mentions 128bit timestamp a couple of times but it doesn't seem to be used in the packet...

pub type Timestamp = u64;

pub type Short = u32;
//    seconds: u16,
//    fraction: u16,
//}

pub trait TimestampTrait<T, H> {
    fn get_seconds(self) -> H;
    fn get_fraction(self) -> H;
    fn set_seconds(self, seconds: H) -> T;
    fn set_fraction(self, fraction: H) -> T;
}

impl TimestampTrait<Timestamp, u32> for Timestamp {
    fn get_seconds(self) -> u32 { (self >> 32) as u32 }
    fn get_fraction(self) -> u32 { (self & 0xffff_ffff) as u32 }
    fn set_seconds(self, seconds: u32) -> Self { ((seconds as u64) << 32) | (self.get_fraction() as u64) }
    fn set_fraction(self, fraction: u32) -> Self { self | (fraction as u64) }
}

impl TimestampTrait<Short, u16> for Short {
    fn get_seconds(self) -> u16 { (self >> 16) as u16 }
    fn get_fraction(self) -> u16 { (self & 0xffff) as u16 }
    fn set_seconds(self, seconds: u16) -> Self { ((seconds as u32) << 16) | (self.get_fraction() as u32) }
    fn set_fraction(self, fraction: u16) -> Self { self | (fraction as u32) }
}

#[derive(Debug)]
pub struct ExtensionField {
    field_type: u16,
    length: u16,
    value: Box<Vec<u8>>,
}

#[derive(Debug,Clone)]
pub struct Packet {
    pub leap_indicator: LeapIndicator,      //2 bits
    pub version: u8,                        //3 bits
    pub mode: Mode,                         //3 bits
    pub stratum: Stratum,                   //8 bits
    pub poll: i8,                           //8 bits
    pub precision: i8,                      //8 bits
    pub root_delay: Short,                  //32 bits
    pub root_dispersion: Short,             //32 bits
    pub reference_id: [u8;4],               //4 bytes (server ip address?)
    pub reference_timestamp: Timestamp,     //64 bits?
    pub origin_timestamp: Timestamp,        //64 bits?
    pub receive_timestamp: Timestamp,       //64 bits?
    pub transit_timestamp: Timestamp,       //64 bits?
    //pub extensions: Vec<Extensions>,      //depends
    pub auth: Option<Auth>                  //32 bits, 128 bits, optional
}
//big endian

impl Packet {
    fn size(&self) -> usize {
        let mut size = 48; 
        if let Some(_) = self.auth {
            size += 20;
        }
        //TODO: EXTENSIONS!!!
        size
    }
}

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

