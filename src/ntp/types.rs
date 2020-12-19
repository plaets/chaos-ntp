use std::convert::{TryFrom,TryInto};
use std::mem::size_of;
use simple_error::SimpleError;
use num_enum::{IntoPrimitive,TryFromPrimitive};

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
        { $x.as_bytes()[..($len)].try_into().unwrap() }
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
    pub key_indentifier: u32,   //32 bits, optional
    pub digest: u128,           //128 bits, optional
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

//this is probably broken 
macro_rules! gen_timestamp_trait {
    ($name:ident, $size:ident, $halfsize:ident) => {
        impl TimestampTrait<$name, $halfsize> for $name {
            fn get_seconds(self) -> $halfsize { (self >> (size_of::<$halfsize>()*8)) as $halfsize }
            fn get_fraction(self) -> $halfsize { self as $halfsize }
            fn set_seconds(self, seconds: $halfsize) -> Self { ((seconds as $size) << (size_of::<$halfsize>()*8)) 
                | (self.get_fraction() as $size) }
            fn set_fraction(self, fraction: $halfsize) -> Self { self | (fraction as $size) }
        }
    }
}

gen_timestamp_trait!(Timestamp, u64, u32);
gen_timestamp_trait!(Short, u32, u16);

#[derive(Debug)]
pub struct ExtensionField {
    pub field_type: u16,
    pub length: u16,
    pub value: Box<Vec<u8>>,
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
    pub fn size(&self) -> usize {
        let mut size = 48; 
        if let Some(_) = self.auth {
            size += 20;
        }
        //TODO: EXTENSIONS!!!
        size
    }
}
