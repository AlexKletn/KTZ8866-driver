use super::super::RegisterBus;

use super::{BiasConfig, Delay};

const BIAS_CONTROL: u8 = 0x09;
const LCD_BOOST_CONFIG: u8 = 0x0c;
const POSITIVE_OUTPUT_CONFIG: u8 = 0x0d;
const NEGATIVE_OUTPUT_CONFIG: u8 = 0x0e;
const BIAS_ARMED: u8 = 0x9c;
const BIAS_ENABLED: u8 = 0x9e;
const BIAS_DISABLED: u8 = 0x98;

pub(crate) fn enable<B: RegisterBus>(
    bus: &mut B,
    config: &BiasConfig,
    delay: &mut impl Delay,
) -> Result<(), B::Error> {
    bus.write(LCD_BOOST_CONFIG, config.boost_register)?;
    bus.write(POSITIVE_OUTPUT_CONFIG, config.positive_output_register)?;
    bus.write(NEGATIVE_OUTPUT_CONFIG, config.negative_output_register)?;
    switch_bias(bus, config, delay, BIAS_ENABLED)
}

pub(crate) fn disable<B: RegisterBus>(
    bus: &mut B,
    config: &BiasConfig,
    delay: &mut impl Delay,
) -> Result<(), B::Error> {
    switch_bias(bus, config, delay, BIAS_DISABLED)
}

fn switch_bias<B: RegisterBus>(
    bus: &mut B,
    config: &BiasConfig,
    delay: &mut impl Delay,
    target: u8,
) -> Result<(), B::Error> {
    bus.write(BIAS_CONTROL, BIAS_ARMED)?;
    delay.delay_ms(config.step_delay_ms);
    bus.write(BIAS_CONTROL, target)
}
