pub trait RegisterBus {
    type Error;

    fn write(&mut self, register: u8, value: u8) -> Result<(), Self::Error>;
}
