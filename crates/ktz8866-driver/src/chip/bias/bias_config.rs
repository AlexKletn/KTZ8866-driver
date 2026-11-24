use core::num::NonZeroU32;

pub struct BiasConfig {
    pub boost_register: u8,
    pub positive_output_register: u8,
    pub negative_output_register: u8,
    pub step_delay_ms: NonZeroU32,
}
