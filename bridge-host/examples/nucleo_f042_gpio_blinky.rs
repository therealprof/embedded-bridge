use std::env;
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serial::prelude::*;

use embedded_hal::digital::v1::OutputPin;

use simplelog::*;

fn main() -> io::Result<()> {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap();

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

    bridge_host::common::assert_version(port.clone());

    let mut pin = bridge_host::gpio::PushPullPin::new("b3".into(), port.clone());

    loop {
        pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(500));

        pin.set_high();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
