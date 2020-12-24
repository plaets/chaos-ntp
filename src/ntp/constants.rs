#[non_exhaustive]
struct ClockSource;
#[allow(dead_code)]
impl ClockSource {
    pub const GOES: [u8;4] = *b"GOES";
    pub const GPS: [u8;4] = *b"GPS\0";
    pub const GAL: [u8;4] = *b"GAL\0";
    pub const PPS: [u8;4] = *b"PPS\0";
    pub const IRIG: [u8;4] = *b"IRIG";
    pub const WWVB: [u8;4] = *b"WWVB";
    pub const DCF: [u8;4] = *b"DCF\0";
    pub const HBG: [u8;4] = *b"HBG\0";
    pub const MSF: [u8;4] = *b"MSF\0";
    pub const JJY: [u8;4] = *b"JJY\0";
    pub const LORC: [u8;4] = *b"LORC";
    pub const TDF: [u8;4] = *b"TDF\0";
    pub const CHU: [u8;4] = *b"CHU\0";
    pub const WWV: [u8;4] = *b"WWV\0";
    pub const WWVH: [u8;4] = *b"WWVH";
    pub const NIST: [u8;4] = *b"NIST";
    pub const ACTS: [u8;4] = *b"ACTS";
    pub const USNO: [u8;4] = *b"USNO";
    pub const PTB: [u8;4] = *b"PTB\0";
}

#[non_exhaustive]
struct KoD;
#[allow(dead_code)]
impl KoD {
    pub const ACST: [u8;4] = *b"ACST";   //the associtaion belongs to a unicast server
    pub const AUTH: [u8;4] = *b"AUTH";   //server authentication failed
    pub const AUTO: [u8;4] = *b"AUTO";   //autokey sequence failed
    pub const BCST: [u8;4] = *b"BCST";   //the association belongs to a broadcast server
    pub const CRYP: [u8;4] = *b"CRYP";   //cyrptographic authentication or identification failed
    pub const DENY: [u8;4] = *b"DENY";   //access denied by remote server
    pub const DROP: [u8;4] = *b"DROP";   //lost peer in symmetric mode
    pub const RSTR: [u8;4] = *b"RSTR";   //access denied due to local policy
    pub const INIT: [u8;4] = *b"INIT";   //the association has not yet synchronized for the first time
    pub const MCST: [u8;4] = *b"MCST";   //the association belongs to a dynamically discovered server
    pub const NKEY: [u8;4] = *b"NKEY";   //no key found. either the key was never installed or is not trusted
    pub const RATE: [u8;4] = *b"RATE";   //rate exceeded, access denied temporarily
    pub const RMOT: [u8;4] = *b"RMOT";   //alteration of associations from a remote host running ntpdc
    pub const STEP: [u8;4] = *b"STEP";   //a step change in system time has occured, but the association has not yet resynchronized
}

//still not sure if i didn't prefer enum
//if there only was a macro such as num_enum that supports arrays and a variant for all the other
//values
#[non_exhaustive]
struct ExtensionFieldType;
#[allow(dead_code)]
impl ExtensionFieldType {
    pub const NOOP: u16 = 0x002;
    pub const UNIQUE: u16 = 0x0104;
    pub const NTS_COOKIE: u16 = 0x0204;
    pub const NTS_COOKIE_PLACEHOLDER: u16 = 0x0304;
    pub const NTP_AUTHENTICATOR: u16 = 0x0404;
    pub const NOOP_RESPONSE: u16 = 0x8002;
    pub const NOOP_ERROR: u16 = 0xc002;
    pub const ASSOCIATION_REQUEST: u16 = 0x0102;
    pub const ASSOCIATION_RESPONSE: u16 = 0x8102;
    pub const ASSOCIATION_ERROR: u16 = 0xc102;
    pub const CERTIFICATE_REQUEST: u16 = 0x0202;
    pub const CERTIFICATE_RESPONSE: u16 = 0x8202;
    pub const CERTIFICATE_ERROR: u16 = 0xc202;
    pub const COOKIE_REQUEST: u16 = 0x0302;
    pub const COOKIE_RESPONSE: u16 = 0x8302;
    pub const COOKIE_ERROR: u16 = 0xc302;
    pub const AUTOKEY_REQUEST: u16 = 0x0402;
    pub const AUTOKEY_RESPONSE: u16 = 0x8402;
    pub const AUTOKEY_ERROR: u16 = 0xc402;
    pub const LEAPSECONDS_REQUEST: u16 = 0x0502;
    pub const LEAPSECONDS_RESPONSE: u16 = 0x8502;
    pub const LEAPSECONDS_ERROR: u16 = 0xc502;
    pub const SIGN_REQUEST: u16 = 0x0602;
    pub const SIGN_RESPONSE: u16 = 0x8602;
    pub const SIGN_ERROR: u16 = 0xc602;
    pub const IFF_IDENTITY_REQUEST: u16 = 0x0702;
    pub const IFF_IDENTITY_RESPONSE: u16 = 0x8702;
    pub const IFF_IDENTITY_ERROR: u16 = 0xc702;
    pub const GQ_IDENTITY_REQUEST: u16 = 0x0802;
    pub const GQ_IDENTITY_RESPONSE: u16 = 0x8802;
    pub const GQ_IDENTITY_ERROR: u16 = 0xc802;
    pub const MV_IDENTITY_REQUEST: u16 = 0x0902;
    pub const MV_IDENTITY_RESPONSE: u16 = 0x8902;
    pub const MV_IDENTITY_ERROR: u16 = 0xc902;
    pub const CHECKSUM_COMPLEMENT: u16 = 0x2005;
}

