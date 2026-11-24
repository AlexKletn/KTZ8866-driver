use core::num::NonZeroU32;

pub trait Delay {
    fn delay_ms(&mut self, milliseconds: NonZeroU32);
}
