#[derive(Default)]
pub struct BacklightConfig {
    pub led_config_1: Option<u8>,
    pub led_config_2: Option<u8>,
    pub dimming: Option<u8>,
    pub led_option_2: Option<u8>,
    pub pwm_ramp: Option<u8>,
}
