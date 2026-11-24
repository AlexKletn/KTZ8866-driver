#[derive(Debug, PartialEq, Eq)]
pub enum DriverError<E> {
    InvalidSinkMask(u8),
    BrightnessOutOfRange(u16),
    Bus(E),
}
