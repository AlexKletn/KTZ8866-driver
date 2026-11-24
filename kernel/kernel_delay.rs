// SPDX-License-Identifier: GPL-2.0-only

use core::num::NonZeroU32;

use kernel::time::{Delta, delay::fsleep};
use ktz8866_driver::Delay;

/// Sleep-based delays for use in a sleepable kernel context.
pub struct KernelDelay;

impl Delay for KernelDelay {
    fn delay_ms(&mut self, milliseconds: NonZeroU32) {
        // fsleep bounds its argument in microseconds, so split longer delays.
        const MAX_SLEEP_MS: u32 = i32::MAX as u32 / 1000;
        let mut remaining_ms = milliseconds.get();
        while remaining_ms > 0 {
            let sleep_ms = remaining_ms.min(MAX_SLEEP_MS);
            fsleep(Delta::from_millis(i64::from(sleep_ms)));
            remaining_ms -= sleep_ms;
        }
    }
}
