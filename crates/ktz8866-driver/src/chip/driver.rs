use super::{
    BacklightConfig, BiasConfig, Delay, DriverError, RegisterBus, bias,
    registers::{
        BRIGHTNESS_LSB, BRIGHTNESS_MSB, DIMMING, LED_CONFIG_1, LED_CONFIG_2, LED_ENABLE,
        LED_ENABLE_BIT, LED_OPTION_2, LED_SINK_MASK, PWM_RAMP,
    },
};

pub struct Ktz8866<B> {
    bus: B,
    sink_mask: u8,
}

impl<B: RegisterBus> Ktz8866<B> {
    pub const MAX_BRIGHTNESS: u16 = 0x7ff;

    pub fn new(bus: B, sink_mask: u8) -> Result<Self, DriverError<B::Error>> {
        if sink_mask == 0 || sink_mask & !LED_SINK_MASK != 0 {
            return Err(DriverError::InvalidSinkMask(sink_mask));
        }
        Ok(Self { bus, sink_mask })
    }

    /// After a bus error, the caller must treat the bias state as unknown.
    pub fn enable_bias(
        &mut self,
        config: &BiasConfig,
        delay: &mut impl Delay,
    ) -> Result<(), DriverError<B::Error>> {
        bias::enable(&mut self.bus, config, delay).map_err(DriverError::Bus)
    }

    /// After a bus error, the caller must treat the bias state as unknown.
    pub fn disable_bias(
        &mut self,
        config: &BiasConfig,
        delay: &mut impl Delay,
    ) -> Result<(), DriverError<B::Error>> {
        bias::disable(&mut self.bus, config, delay).map_err(DriverError::Bus)
    }

    pub fn configure_backlight(
        &mut self,
        config: &BacklightConfig,
    ) -> Result<(), DriverError<B::Error>> {
        for (register, value) in [
            (LED_CONFIG_1, config.led_config_1),
            (LED_CONFIG_2, config.led_config_2),
            (DIMMING, config.dimming),
            (LED_OPTION_2, config.led_option_2),
            (PWM_RAMP, config.pwm_ramp),
        ] {
            if let Some(value) = value {
                self.bus.write(register, value).map_err(DriverError::Bus)?;
            }
        }
        Ok(())
    }

    pub fn set_brightness(&mut self, level: u16) -> Result<(), DriverError<B::Error>> {
        self.write_brightness(level)?;
        self.set_leds_enabled(level != 0)
    }

    // Systems with two chips can stage both levels before changing their LED enables.
    pub fn write_brightness(&mut self, level: u16) -> Result<(), DriverError<B::Error>> {
        if level > Self::MAX_BRIGHTNESS {
            return Err(DriverError::BrightnessOutOfRange(level));
        }

        self.bus
            .write(BRIGHTNESS_LSB, (level & 0x7) as u8)
            .map_err(DriverError::Bus)?;
        self.bus
            .write(BRIGHTNESS_MSB, (level >> 3) as u8)
            .map_err(DriverError::Bus)
    }

    pub fn set_leds_enabled(&mut self, enabled: bool) -> Result<(), DriverError<B::Error>> {
        let enable = if enabled {
            LED_ENABLE_BIT | self.sink_mask
        } else {
            0
        };
        self.bus.write(LED_ENABLE, enable).map_err(DriverError::Bus)
    }
}

#[cfg(test)]
#[path = "driver.tests.rs"]
mod tests;
