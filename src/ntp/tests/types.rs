use crate::ntp::types::{Timestamp,Short,TimestampTrait};

#[test]
fn timestamp() {
    let timestamp = Timestamp::from(0x12345678_9abcdef0);

    assert_eq!(0x12345678, timestamp.get_seconds());
    assert_eq!(0x9abcdef0, timestamp.get_fraction());
    assert_eq!(0x731_9abcdef0 as u64, u64::from(timestamp.set_seconds(0x731)));
    assert_eq!(0x731, timestamp.set_seconds(0x731).get_seconds());

    assert_eq!(0x9abcdef0, timestamp.set_seconds(0x731).get_fraction());
    assert_eq!(0x12345678, timestamp.set_fraction(0x52141).get_seconds());
    assert_eq!(0x12345678_00052141 as u64, u64::from(timestamp.set_fraction(0x52141)));
    assert_eq!(0x52141, timestamp.set_fraction(0x52141).get_fraction());


    assert_eq!(Timestamp::from(0).set_seconds(0xe38c4fd4).set_fraction(0xd7472dcd).into_utc_datetime().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
        "2020-12-22T10:58:28.840929853Z");
}

//loosy - fraction_from_nanoseconds(fraction_as_nanoseconds) != fraction
#[test]
fn nanoseconds_conversions_timestamp() {
    let timestamp = Timestamp::from(0);

    assert_eq!(840929853, timestamp.set_fraction(3611766221).fraction_as_nanoseconds());
    assert_eq!(912855987, timestamp.set_fraction(3920686612).fraction_as_nanoseconds());

    assert_eq!(3611766216, timestamp.fraction_from_nanoseconds(840929853).unwrap().get_fraction());
    assert_eq!(3920686610, timestamp.fraction_from_nanoseconds(912855987).unwrap().get_fraction());
}

#[test]
fn short() {
    let short = Short::from(0x1234_5678);

    assert_eq!(0x1234, short.get_seconds());
    assert_eq!(0x5678, short.get_fraction());
    assert_eq!(0x731_5678 as u32, u32::from(short.set_seconds(0x731)));
    assert_eq!(0x731, short.set_seconds(0x731).get_seconds());

    assert_eq!(0x5678, short.set_seconds(0x731).get_fraction());
    assert_eq!(0x1234, short.set_fraction(0x5214).get_seconds());
    assert_eq!(0x1234_5214 as u32, u32::from(short.set_fraction(0x5214)));
    assert_eq!(0x5214, short.set_fraction(0x5214).get_fraction());

    assert_eq!(Short::from_duration(chrono::Duration::seconds(15)).unwrap().get_seconds(), 15);
    assert_eq!(Short::from_duration(chrono::Duration::seconds(15)).unwrap().get_fraction(), 0);
    assert_eq!(Short(0).set_seconds(15).into_duration(), chrono::Duration::seconds(15));
}

