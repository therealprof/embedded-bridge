use embedded_hal::blocking::spi;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crate::io::{send_clear, send_spi_init, send_spi_write};

pub struct SPI<T> {
    ident: String,
    channel: Arc<Mutex<Box<T>>>,
}

impl<T> SPI<T>
where
    T: Read + Write,
{
    pub fn new(
        ident: String,
        sck: String,
        miso: String,
        mosi: String,
        speed: u32,
        channel: Arc<Mutex<Box<T>>>,
    ) -> Self {
        send_clear(&mut *channel.lock().unwrap()).ok();
        send_spi_init(
            &mut *channel.lock().unwrap(),
            &ident,
            &sck,
            &miso,
            &mosi,
            speed,
        )
        .ok();

        SPI { ident, channel }
    }
}

impl<T> spi::Write<u8> for SPI<T>
where
    T: Read + Write,
{
    type Error = std::io::Error;

    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        send_spi_write(&mut *self.channel.lock().unwrap(), &self.ident, bytes)
    }
}
