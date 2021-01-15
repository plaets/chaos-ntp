use std::convert::{TryFrom,TryInto,From,Into};
use std::num::TryFromIntError;
use std::mem::size_of;
use simple_error::SimpleError;
use num_enum::{IntoPrimitive,TryFromPrimitive};
use derive_more::{Add,Mul,From,Into,Deref,DerefMut,LowerHex};

#[derive(Debug,Eq,PartialEq,Clone,Copy,IntoPrimitive,TryFromPrimitive)]
#[repr(u8)]
pub enum LeapIndicator {
    NoWarning = 0,
    LastMinute61Seconds = 1,
    LastMinute59Seconds = 2,
    Unknown = 3,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
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

#[derive(Debug,Clone,Copy)]
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
//update: i think i know why

#[derive(Debug,Clone,Eq,PartialEq,Ord,PartialOrd,Copy,Add,Mul,Deref,DerefMut,From,Into,LowerHex)]
pub struct Timestamp(pub u64);

#[derive(Debug,Clone,Eq,PartialEq,Ord,PartialOrd,Copy,Add,Mul,Deref,DerefMut,From,Into,LowerHex)]
pub struct Short(pub u32);

pub trait TimestampTrait<T, H> {
    type HalfSize;

    fn get_seconds(self) -> H;
    fn get_fraction(self) -> H;
    fn fraction_as_nanoseconds(self) -> u32;

    fn set_seconds(self, seconds: H) -> T;
    fn set_fraction(self, fraction: H) -> T;
    fn fraction_from_nanoseconds(self, fraction: u32) -> Result<T, TryFromIntError>;
}

impl Timestamp {
    //seems like chrono does not handle leap seconds yet...
    //is this really an issue?
    pub fn into_utc_datetime(self) -> chrono::DateTime<chrono::offset::Utc> {
        //2208988800 - 1970-1900 as seconds
        let ntp_epoch = chrono::naive::NaiveDate::from_ymd(1900, 1, 1).and_hms(0, 0, 0);
        let seconds = chrono::Duration::seconds(self.get_seconds().into());
        let nanoseconds = chrono::Duration::nanoseconds(self.fraction_as_nanoseconds().into());
        chrono::DateTime::from_utc(ntp_epoch + seconds + nanoseconds, chrono::offset::Utc)
    }

    pub fn from_utc_datetime(datetime: chrono::DateTime<chrono::offset::Utc>) -> Result<Self,TryFromIntError> {
        let ntp_epoch = chrono::naive::NaiveDate::from_ymd(1900, 1, 1).and_hms(0, 0, 0);
        let duration = datetime.naive_utc()-ntp_epoch;
        let nanoseconds = duration.num_nanoseconds().unwrap_or(0)
            - chrono::Duration::seconds(duration.num_seconds()).num_nanoseconds().unwrap_or(0);
        Ok(Self::from((duration.num_seconds() as u64) << 32u32).fraction_from_nanoseconds(nanoseconds.try_into()?)?)
    }
}

impl Short {
    pub fn into_duration(self) -> chrono::Duration {
        chrono::Duration::seconds(self.get_seconds().into()) + 
            chrono::Duration::nanoseconds(self.fraction_as_nanoseconds().into())
    }

    pub fn from_duration(duration: chrono::Duration) -> Result<Self,TryFromIntError> {
        Ok(Self((duration.num_seconds() as u32) << 16u16).fraction_from_nanoseconds(
                duration.checked_sub(&chrono::Duration::seconds(duration.num_seconds()))
                    .unwrap().num_nanoseconds().unwrap().try_into()?
            )?)
    }
}

//this probably is broken 
//update: even more broken now
macro_rules! gen_timestamp_trait {
    ($name:ident, $size:ident, $halfsize:ident) => {
        impl TimestampTrait<$name, $halfsize> for $name {
            type HalfSize = $halfsize;

            fn get_seconds(self) -> $halfsize { 
                ($size::from(self) >> ((size_of::<$halfsize>() as $halfsize)*8)) as $halfsize 
            }

            fn get_fraction(self) -> $halfsize { 
                $size::from(self) as $halfsize 
            }

            fn set_seconds(self, seconds: $halfsize) -> Self { 
                (((seconds as $size) << (size_of::<$halfsize>()*8)) 
                | $size::from((self.get_fraction()))).into() 
            }

            fn set_fraction(self, fraction: $halfsize) -> Self { 
                ($size::from(self) & (((1 << size_of::<$halfsize>()*8)-1) << size_of::<$halfsize>()*8) 
                 | (fraction as $size)).into() 
            }

            //loosy - fraction_from_nanoseconds(fraction_as_nanoseconds) != fraction
            fn fraction_as_nanoseconds(self) -> u32 {
                //u32::try_from((((self.get_fraction() as u64)*1_000_000_000u64)/(1u64 << 32))).unwrap()
                u32::try_from(((self.get_fraction() as u64)*1_000_000_000u64) >> 32).unwrap()
            }

            fn fraction_from_nanoseconds(self, nanoseconds: u32) -> Result<Self, TryFromIntError> {
                (((nanoseconds as u64) << 32u64)/1_000_000_000u64)
                    .try_into().and_then(|f| Ok(self.set_fraction(f)))
            }
        }
    }
}

gen_timestamp_trait!(Timestamp, u64, u32);
gen_timestamp_trait!(Short, u32, u16);

#[derive(Debug,Clone)]
pub struct ExtensionField {
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
    pub extensions: Option<Vec<ExtensionField>>, //depends
    pub auth: Option<Auth>                  //32 bits, 128 bits, optional
}
//big endian

impl Packet {
    //TODO: maybe all of this should be moved to the parser
    pub const BASE_SIZE: usize = 48;
    pub const AUTH_SIZE: usize = 20;
    pub const EXT_HEAD_SIZE: usize = 4;
    pub const MAX_SIZE: usize = 65527; //max udp payload

    pub fn size(&self) -> usize {
        let mut size = Self::BASE_SIZE; 
        if self.auth.is_some() {
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

