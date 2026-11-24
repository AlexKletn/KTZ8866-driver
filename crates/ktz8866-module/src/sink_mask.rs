/// Firmware specifies a count of contiguous LED current sinks.
pub fn sink_mask(count: u32) -> Option<u8> {
    (1..=6).contains(&count).then(|| (1 << count) - 1)
}

#[cfg(test)]
#[path = "sink_mask.tests.rs"]
mod tests;
