use embedded_hal::digital::v2::OutputPin;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crate::io::{send_clear, send_gpio_high, send_gpio_init_pp, send_gpio_low};

pub struct PushPullPin<T> {
    pinname: String,
    channel: Arc<Mutex<Box<T>>>,
}

impl<T> PushPullPin<T>
where
    T: Read + Write,
{
    pub fn new(pinname: String, channel: Arc<Mutex<Box<T>>>) -> Result<Self, ()> {
        send_clear(&mut *channel.lock().unwrap()).ok();
        let res = send_gpio_init_pp(&mut *channel.lock().unwrap(), &pinname);
        res.map(|_| PushPullPin { channel, pinname })
            .map_err(|_| ())
    }
}

impl<T> OutputPin for PushPullPin<T>
where
    T: Read + Write,
{
    type Error = ();

    fn set_high(&mut self) -> Result<(), ()> {
        send_gpio_high(&mut *self.channel.lock().unwrap(), &self.pinname).map_err(|_| ())
    }

    fn set_low(&mut self) -> Result<(), ()> {
        send_gpio_low(&mut *self.channel.lock().unwrap(), &self.pinname).map_err(|_| ())
    }
}
