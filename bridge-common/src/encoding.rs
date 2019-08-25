use serde::{Deserialize, Serialize};

pub const VERSION: u8 = 5;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Request<'p> {
    Version,
    /// Clear all receive buffers, a good idea to use before critical operations
    Clear,
    /// Reset the target into a clean state (may not be supported)
    Reset,
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
    SPIInit {
        sck_pin: &'p str,
        miso_pin: &'p str,
        mosi_pin: &'p str,
        speed: u32,
    },
    SPIWrite {
        ident: &'p str,
        data: &'p [u8],
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Reply<'a> {
    Ok,
    Incomplete,
    NotImplemented,
    Version { version: u8 },
    VerboseErr { err: &'a str },
    ReceiveErr { bytes: u8 },
    Err { bytes: u8 },
}

pub fn version() -> Request<'static> {
    Request::Version
}

pub fn current_version() -> Reply<'static> {
    Reply::Version { version: VERSION }
}

pub fn clear() -> Request<'static> {
    Request::Clear
}

pub fn reset() -> Request<'static> {
    Request::Reset
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

pub fn spi_init<'p>(sck_pin: &'p str, miso_pin: &'p str, mosi_pin: &'p str, speed: u32) -> Request<'p> {
    Request::SPIInit {
        sck_pin,
        miso_pin,
        mosi_pin,
        speed,
    }
}

pub fn spi_write<'p>(ident: &'p str, data: &'p [u8]) -> Request<'p> {
    Request::SPIWrite {
        ident,
        data,
    }
}
