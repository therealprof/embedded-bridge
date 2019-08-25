use std::env;
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serial::prelude::*;

use embedded_hal::digital::v1::OutputPin;

use ssd1306::mode::TerminalMode;
use ssd1306::Builder;

use core::fmt::Write;

fn main() -> io::Result<()> {
    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
        interact(&mut port)?;
    }

    Ok(())
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(2000))?;

    let port = Arc::new(Mutex::new(Box::from(port)));

    let mut pin = bridge_host::gpio::PushPullPin::new("b3".into(), port.clone());
    let i2c =
        bridge_host::i2c::I2C::new("i2c1".into(), "f1".into(), "f0".into(), 400, port.clone());

    use ssd1306::displayrotation::DisplayRotation;
    let mut disp: TerminalMode<_> = Builder::new().with_i2c_addr(0x3c).connect_i2c(i2c).into();

    let _ = disp.set_rotation(DisplayRotation::Rotate180);
    disp.init().unwrap();
    let _ = disp.clear();

    loop {
        pin.set_low();

        for c in 97..123 {
            let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }
        for c in 65..91 {
            let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }

        pin.set_high();
    }
}
