use std::env;
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use smart_leds::{SmartLedsWrite, RGB};
use apa102_spi::*;

use serial::prelude::*;

use simplelog::*;

fn main() -> io::Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        simplelog::TerminalMode::Mixed,
    )
    .unwrap();

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

    let spi =
        bridge_host::spi::SPI::new("spi1".into(), "a5".into(), "a6".into(), "a7".into(), 1000, port.clone());


    let mut apa = Apa102::new(spi);
    let data: [RGB<u8>; 4] = [(0, 0, 0).into(), (255, 0, 0).into(), (0, 255, 0).into(), (0, 0, 255).into()];

    apa.write(data.iter().cloned()).unwrap();

    Ok(())
}
