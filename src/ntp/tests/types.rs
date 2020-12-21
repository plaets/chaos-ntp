use crate::ntp::types::{Timestamp,Short,TimestampTrait};

#[test]
fn timestamp() {
    let timestamp = Timestamp::from(0x12345678_9abcdef0);

    assert_eq!(0x12345678, timestamp.get_seconds());
    assert_eq!(0x9abcdef0, timestamp.get_fraction());
    assert_eq!(0x731_9abcdef0 as u64, timestamp.set_seconds(0x731).into());
    assert_eq!(0x731, timestamp.set_seconds(0x731).get_seconds());

    assert_eq!(0x9abcdef0, timestamp.set_seconds(0x731).get_fraction());
    assert_eq!(0x12345678, timestamp.set_fraction(0x52141).get_seconds());
    assert_eq!(0x12345678_00052141 as u64, timestamp.set_fraction(0x52141).into());
    assert_eq!(0x52141, timestamp.set_fraction(0x52141).get_fraction());
}

#[test]
fn short() {
    let short = Short::from(0x1234_5678);

    assert_eq!(0x1234, short.get_seconds());
    assert_eq!(0x5678, short.get_fraction());
    assert_eq!(0x731_5678 as u32, short.set_seconds(0x731).into());
    assert_eq!(0x731, short.set_seconds(0x731).get_seconds());

    assert_eq!(0x5678, short.set_seconds(0x731).get_fraction());
    assert_eq!(0x1234, short.set_fraction(0x5214).get_seconds());
    assert_eq!(0x1234_5214 as u32, short.set_fraction(0x5214).into());
    assert_eq!(0x5214, short.set_fraction(0x5214).get_fraction());
}

