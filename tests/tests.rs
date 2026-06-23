use chinese_variant::ChineseVariant;

#[test]
fn is_traditional_or_simple() {
    assert!(ChineseVariant::Simple.is_simple());
    assert!(!ChineseVariant::Traditional.is_simple());

    assert!(ChineseVariant::Traditional.is_traditional());
    assert!(!ChineseVariant::Simple.is_traditional());
}

#[cfg(feature = "enum-ordinalize")]
#[test]
fn ordinalize() {
    use enum_ordinalize::Ordinalize;

    assert_eq!(0u8, ChineseVariant::Traditional.ordinal());

    assert_eq!(Some(ChineseVariant::Traditional), ChineseVariant::from_ordinal(0u8));
}
