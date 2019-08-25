use embedded_hal::blocking::i2c;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crate::io::{send_clear, send_i2c_init, send_i2c_write};

pub struct I2C<T> {
    ident: String,
    channel: Arc<Mutex<Box<T>>>,
}

impl<T> I2C<T>
where
    T: Read + Write,
{
    pub fn new(
        ident: String,
        scl: String,
        sda: String,
        speed: u32,
        channel: Arc<Mutex<Box<T>>>,
    ) -> Self {
        send_clear(&mut *channel.lock().unwrap()).ok();
        send_i2c_init(&mut *channel.lock().unwrap(), &ident, &scl, &sda, speed).ok();

        I2C { ident, channel }
    }
}

impl<T> i2c::Write for I2C<T>
where
    T: Read + Write,
{
    type Error = std::io::Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        send_i2c_write(&mut *self.channel.lock().unwrap(), &self.ident, addr, bytes)
    }
}
