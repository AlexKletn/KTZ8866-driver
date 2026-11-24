// SPDX-License-Identifier: GPL-2.0-only

use core::marker::PhantomData;

use kernel::{
    bindings,
    device::Bound,
    error::{Error, to_result},
    i2c::I2cClient,
};
use ktz8866_driver::RegisterBus;

/// Register access through a bound Linux I²C client. Writes may sleep.
pub struct I2cRegisterBus<'a> {
    client: *const bindings::i2c_client,
    lifetime: PhantomData<&'a bindings::i2c_client>,
}

impl<'a> I2cRegisterBus<'a> {
    /// Borrow the client for the lifetime of the register bus.
    pub fn new(client: &'a I2cClient<Bound>) -> Self {
        let client = core::ptr::from_ref(client).cast::<bindings::i2c_client>();
        // SAFETY: I2cClient is a transparent wrapper and the bound borrow keeps it live.
        unsafe { Self::from_raw(client) }
    }

    /// # Safety
    ///
    /// The client must stay valid and registered for the lifetime of the bus.
    pub unsafe fn from_raw(client: *const bindings::i2c_client) -> Self {
        Self {
            client,
            lifetime: PhantomData,
        }
    }
}

impl RegisterBus for I2cRegisterBus<'_> {
    type Error = Error;

    fn write(&mut self, register: u8, value: u8) -> Result<(), Self::Error> {
        // SAFETY: The bus holds a live client for the synchronous transfer.
        let result = unsafe { bindings::i2c_smbus_write_byte_data(self.client, register, value) };
        to_result(result)
    }
}
