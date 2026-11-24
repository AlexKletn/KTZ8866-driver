use super::sink_mask;

#[test]
fn accepts_only_the_six_physical_sinks() {
    assert_eq!(sink_mask(0), None);
    assert_eq!(sink_mask(1), Some(0x01));
    assert_eq!(sink_mask(4), Some(0x0f));
    assert_eq!(sink_mask(6), Some(0x3f));
    assert_eq!(sink_mask(7), None);
}
