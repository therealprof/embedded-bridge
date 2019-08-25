use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Request<'p> {
    GpioInitPP {
        pin: &'p str,
    },
    GpioSetHigh {
        pin: &'p str,
    },
    GpioSetLow {
        pin: &'p str,
    },
    GpioToggle {
        pin: &'p str,
    },
    I2CInit {
        scl_pin: &'p str,
        sda_pin: &'p str,
        speed: u32,
    },
    I2CWrite {
        ident: &'p str,
        address: u8,
        data: &'p [u8],
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Reply<'a> {
    Ok,
    Incomplete,
    NotImplemented,
    VerboseErr { err: &'a str },
    ReceiveErr { bytes: u8 },
    Err { bytes: u8 },
}

pub fn gpio_init_pp(pin: &str) -> Request {
    Request::GpioInitPP { pin }
}

pub fn gpio_setlow(pin: &str) -> Request {
    Request::GpioSetLow { pin }
}

pub fn gpio_sethigh(pin: &str) -> Request {
    Request::GpioSetHigh { pin }
}

pub fn gpio_toggle(pin: &str) -> Request {
    Request::GpioToggle { pin }
}

pub fn i2c_init<'p>(scl_pin: &'p str, sda_pin: &'p str, speed: u32) -> Request<'p> {
    Request::I2CInit {
        scl_pin,
        sda_pin,
        speed,
    }
}

pub fn i2c_write<'p>(ident: &'p str, address: u8, data: &'p [u8]) -> Request<'p> {
    Request::I2CWrite {
        ident,
        address,
        data,
    }
}
