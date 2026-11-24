// SPDX-License-Identifier: GPL-2.0-only

//! Rust register operations for the standalone KTZ8866 module.

// The standalone backlight consumes only part of the shared chip API.
#[allow(dead_code, missing_docs, unreachable_pub)]
#[path = "../ktz8866-driver/src/chip/mod.rs"]
mod chip;
#[path = "src/sink_mask.rs"]
mod sink_mask;

use chip::{DriverError, Ktz8866};
use kernel::{bindings, error::code::EINVAL};

#[allow(dead_code, unreachable_pub)]
#[path = "../../kernel/i2c_register_bus.rs"]
mod i2c_register_bus;

extern crate self as ktz8866_driver;
pub use chip::RegisterBus;
pub use sink_mask::sink_mask;

use i2c_register_bus::I2cRegisterBus;

/// The Linux backlight class uses the same range as the chip core.
#[unsafe(no_mangle)]
pub extern "C" fn ktz8866_rust_max_brightness() -> u16 {
    Ktz8866::<I2cRegisterBus<'_>>::MAX_BRIGHTNESS
}

/// Parse the firmware sink count before the C probe enables any resources.
#[unsafe(no_mangle)]
pub extern "C" fn ktz8866_rust_sink_mask(sinks: u32) -> i32 {
    sink_mask(sinks).map_or(EINVAL.to_errno(), i32::from)
}

/// # Safety
///
/// The caller must provide a live I²C client in a sleepable context.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ktz8866_rust_set_brightness(
    client: *const bindings::i2c_client,
    mask: u8,
    brightness: u16,
) -> i32 {
    // SAFETY: The C callback holds the client live for the synchronous writes.
    let bus = unsafe { I2cRegisterBus::from_raw(client) };
    match Ktz8866::new(bus, mask).and_then(|mut driver| driver.set_brightness(brightness)) {
        Ok(()) => 0,
        Err(DriverError::Bus(error)) => error.to_errno(),
        Err(_) => EINVAL.to_errno(),
    }
}
