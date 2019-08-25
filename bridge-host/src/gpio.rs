use embedded_hal::digital::v1::OutputPin;
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
    pub fn new(pinname: String, channel: Arc<Mutex<Box<T>>>) -> Self {
        send_clear(&mut *channel.lock().unwrap()).ok();
        send_gpio_init_pp(&mut *channel.lock().unwrap(), &pinname).ok();

        PushPullPin { channel, pinname }
    }
}

impl<T> OutputPin for PushPullPin<T>
where
    T: Read + Write,
{
    fn set_high(&mut self) {
        send_gpio_high(&mut *self.channel.lock().unwrap(), &self.pinname).ok();
    }

    fn set_low(&mut self) {
        send_gpio_low(&mut *self.channel.lock().unwrap(), &self.pinname).ok();
    }
}
