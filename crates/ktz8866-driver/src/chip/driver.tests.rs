use super::{DriverError, Ktz8866, RegisterBus};

#[derive(Default)]
struct FakeBus {
    writes: [(u8, u8); 8],
    count: usize,
    fail_at: Option<usize>,
}

impl RegisterBus for FakeBus {
    type Error = &'static str;

    fn write(&mut self, register: u8, value: u8) -> Result<(), Self::Error> {
        if self.fail_at == Some(self.count) {
            return Err("I2C write failed");
        }
        self.writes[self.count] = (register, value);
        self.count += 1;
        Ok(())
    }
}

#[test]
fn programs_brightness_before_enabling_selected_sinks_and_blanks_at_zero() {
    let mut driver = Ktz8866::new(FakeBus::default(), 0x0f).unwrap();

    driver.set_brightness(0x7ff).unwrap();
    driver.set_brightness(0).unwrap();

    assert_eq!(
        driver.bus.writes[..driver.bus.count],
        [
            (0x04, 0x07),
            (0x05, 0xff),
            (0x08, 0x4f),
            (0x04, 0),
            (0x05, 0),
            (0x08, 0)
        ]
    );
}

#[test]
fn rejects_invalid_settings_without_writing_to_the_bus() {
    for mask in [0, 0x40, 0xff] {
        assert!(matches!(
            Ktz8866::new(FakeBus::default(), mask),
            Err(DriverError::InvalidSinkMask(rejected)) if rejected == mask
        ));
    }

    let mut driver = Ktz8866::new(FakeBus::default(), 0x3f).unwrap();
    assert_eq!(
        driver.set_brightness(2048),
        Err(DriverError::BrightnessOutOfRange(2048))
    );
    assert_eq!(driver.bus.count, 0);
}

#[test]
fn stops_before_enabling_leds_when_a_brightness_write_fails() {
    let bus = FakeBus {
        fail_at: Some(1),
        ..FakeBus::default()
    };
    let mut driver = Ktz8866::new(bus, 0x0f).unwrap();

    assert_eq!(
        driver.set_brightness(100),
        Err(DriverError::Bus("I2C write failed"))
    );
    assert_eq!(&driver.bus.writes[..driver.bus.count], &[(0x04, 4)]);
}

#[test]
fn configures_bias_and_waits_at_each_transition() {
    let mut driver = Ktz8866::new(FakeBus::default(), 0x0f).unwrap();
    let mut delay = FakeDelay::default();
    let config = bias_config();

    driver.enable_bias(&config, &mut delay).unwrap();
    driver.disable_bias(&config, &mut delay).unwrap();

    assert_eq!(
        &driver.bus.writes[..driver.bus.count],
        &[
            (0x0c, 0x2e),
            (0x0d, 0x24),
            (0x0e, 0x24),
            (0x09, 0x9c),
            (0x09, 0x9e),
            (0x09, 0x9c),
            (0x09, 0x98),
        ]
    );
    assert_eq!(delay.milliseconds, [5, 5]);
}

#[test]
fn bias_transition_propagates_failure_before_waiting() {
    let bus = FakeBus {
        fail_at: Some(3),
        ..FakeBus::default()
    };
    let mut driver = Ktz8866::new(bus, 0x0f).unwrap();
    let mut delay = FakeDelay::default();

    assert_eq!(
        driver.enable_bias(&bias_config(), &mut delay),
        Err(DriverError::Bus("I2C write failed"))
    );
    assert_eq!(driver.bus.count, 3);
    assert_eq!(delay.count, 0);
}

#[derive(Default)]
struct FakeDelay {
    milliseconds: [u32; 2],
    count: usize,
}

impl super::Delay for FakeDelay {
    fn delay_ms(&mut self, milliseconds: core::num::NonZeroU32) {
        self.milliseconds[self.count] = milliseconds.get();
        self.count += 1;
    }
}

fn bias_config() -> super::BiasConfig {
    super::BiasConfig {
        boost_register: 0x2e,
        positive_output_register: 0x24,
        negative_output_register: 0x24,
        step_delay_ms: core::num::NonZeroU32::new(5).unwrap(),
    }
}

#[test]
fn applies_only_selected_led_settings_in_initialization_order() {
    let mut driver = Ktz8866::new(FakeBus::default(), 0x0f).unwrap();
    let config = super::BacklightConfig {
        led_config_1: Some(0xdb),
        led_config_2: Some(0xed),
        dimming: Some(0xaa),
        pwm_ramp: Some(0),
        ..Default::default()
    };

    driver.configure_backlight(&config).unwrap();

    assert_eq!(
        &driver.bus.writes[..driver.bus.count],
        &[(0x02, 0xdb), (0x03, 0xed), (0x14, 0xaa), (0x15, 0)]
    );
}

#[test]
fn stops_configuring_leds_after_a_bus_failure() {
    let bus = FakeBus {
        fail_at: Some(1),
        ..FakeBus::default()
    };
    let mut driver = Ktz8866::new(bus, 0x0f).unwrap();
    let config = super::BacklightConfig {
        led_config_1: Some(0xdb),
        led_config_2: Some(0xed),
        dimming: Some(0xaa),
        ..Default::default()
    };

    assert_eq!(
        driver.configure_backlight(&config),
        Err(DriverError::Bus("I2C write failed"))
    );
    assert_eq!(&driver.bus.writes[..driver.bus.count], &[(0x02, 0xdb)]);
}
