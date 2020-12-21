use std::convert::{TryFrom,TryInto,From,Into};
use std::mem::size_of;
use simple_error::SimpleError;
use num_enum::{IntoPrimitive,TryFromPrimitive};
use derive_more::{Add,Mul,From,Into,Deref,DerefMut,LowerHex};

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
pub enum KoD {
    ACST,   //the associtaion belongs to a unicast server
    AUTH,   //server authentication failed
    AUTO,   //autokey sequence failed
    BCST,   //the association belongs to a broadcast server
    CRYP,   //cyrptographic authentication or identification failed
    DENY,   //access denied by remote server
    DROP,   //lost peer in symmetric mode
    RSTR,   //access denied due to local policy
    INIT,   //the association has not yet synchronized for the first time
    MCST,   //the association belongs to a dynamically discovered server
    NKEY,   //no key found. either the key was never installed or is not trusted
    RATE,   //rate exceeded, access denied temporarily
    RMOT,   //alteration of associations from a remote host running ntpdc
    STEP,   //a step change in system time has occured, but the association has not yet resynchronized
    Unknown([u8;4]),
}

impl Into<[u8;4]> for KoD {
    fn into(self) -> [u8;4] {
        match self {
            Self::ACST => *b"ACST",
            Self::AUTH => *b"AUTH",
            Self::AUTO => *b"AUTO",
            Self::BCST => *b"BCST",
            Self::CRYP => *b"CRYP",
            Self::DENY => *b"DENY",
            Self::DROP => *b"DROP",
            Self::RSTR => *b"RSTR",
            Self::INIT => *b"INIT",
            Self::MCST => *b"MCST",
            Self::NKEY => *b"NKEY",
            Self::RATE => *b"RATE",
            Self::RMOT => *b"RMOT",
            Self::STEP => *b"STEP",
            Self::Unknown(data) => data
        }
    }
}

//this was never tested so far and probably is not very ergonomic
impl From<&[u8;4]> for KoD {
    fn from(data: &[u8;4]) -> KoD {
        match data {
            b"ACST" => Self::ACST,
            b"AUTH" => Self::AUTH,
            b"AUTO" => Self::AUTO,
            b"BCST" => Self::BCST,
            b"CRYP" => Self::CRYP,
            b"DENY" => Self::DENY,
            b"DROP" => Self::DROP,
            b"RSTR" => Self::RSTR,
            b"INIT" => Self::INIT,
            b"MCST" => Self::MCST,
            b"NKEY" => Self::NKEY,
            b"RATE" => Self::RATE,
            b"RMOT" => Self::RMOT,
            b"STEP" => Self::STEP,
            _ => Self::Unknown(data.clone())
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
//update: so apparently while this is not used in the packet, it can be used in the server/client
//still not sure why, how am i supposed to know from which era did the packet come from, should i
//just assume that it came from my era?

#[derive(Debug,Clone,Eq,PartialEq,Ord,PartialOrd,Copy,Add,Mul,Deref,DerefMut,From,Into,LowerHex)]
pub struct Timestamp(pub u64);

#[derive(Debug,Clone,Eq,PartialEq,Ord,PartialOrd,Copy,Add,Mul,Deref,DerefMut,From,Into,LowerHex)]
pub struct Short(pub u32);

pub trait TimestampTrait<T, H> {
    type HalfSize;

    fn get_seconds(self) -> H;
    fn get_fraction(self) -> H;
    fn set_seconds(self, seconds: H) -> T;
    fn set_fraction(self, fraction: H) -> T;
}

//this probably is broken 
//update: even more broken now
macro_rules! gen_timestamp_trait {
    ($name:ident, $size:ident, $halfsize:ident) => {
        impl TimestampTrait<$name, $halfsize> for $name {
            type HalfSize = $halfsize;

            fn get_seconds(self) -> $halfsize { ($size::from(self) >> ((size_of::<$halfsize>() as $halfsize)*8)) as $halfsize }
            fn get_fraction(self) -> $halfsize { $size::from(self) as $halfsize }
            fn set_seconds(self, seconds: $halfsize) -> Self { (((seconds as $size) << (size_of::<$halfsize>()*8)) 
                | $size::from((self.get_fraction()))).into() }
            fn set_fraction(self, fraction: $halfsize) -> Self { 
                ($size::from(self) & (((1 << size_of::<$halfsize>()*8)-1) << size_of::<$halfsize>()*8) 
                 | (fraction as $size)).into() }
        }
    }
}

gen_timestamp_trait!(Timestamp, u64, u32);
gen_timestamp_trait!(Short, u32, u16);

#[derive(Debug,Clone)]
pub struct Extension {
    pub field_type: u16,
    //pub length: u16,
    pub value: Vec<u8>,
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
    pub extensions: Option<Vec<Extension>>, //depends
    pub auth: Option<Auth>                  //32 bits, 128 bits, optional
}
//big endian

impl Packet {
    pub const BASE_SIZE: usize = 48;
    pub const AUTH_SIZE: usize = 20;
    pub const EXT_HEAD_SIZE: usize = 4;

    pub fn size(&self) -> usize {
        let mut size = Self::BASE_SIZE; 
        if let Some(_) = self.auth {
            size += Self::AUTH_SIZE;
        }
        if let Some(extensions) = &self.extensions {
            for n in extensions {
                size += Self::EXT_HEAD_SIZE;
                size += n.value.len();
            }
        }
        size
    }
}

