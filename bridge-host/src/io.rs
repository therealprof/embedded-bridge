use bridge_common::encoding::{
    clear, gpio_init_pp, gpio_sethigh, gpio_setlow, gpio_toggle, i2c_init, i2c_write, reset,
    spi_init, spi_write, version, Reply, Request,
};
use heapless::{consts::*, Vec};
use postcard::{from_bytes, to_vec};
use std::io::{self, Read, Write};
use std::ops::Deref;

type BufferLength = U64;

pub fn send_version<T: Read + Write>(port: &mut T) -> io::Result<u8> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&version()).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    match res.unwrap() {
        Reply::Version { version } => Ok(version),
        _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
    }
}

pub fn send_clear<T: Read + Write>(port: &mut T) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&clear()).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_reset<T: Read + Write>(port: &mut T) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&reset()).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_gpio_init_pp<T: Read + Write>(port: &mut T, pin: &str) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&gpio_init_pp(pin)).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_gpio_toggle<T: Read + Write>(port: &mut T, pin: &str) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&gpio_toggle(pin)).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_gpio_high<T: Read + Write>(port: &mut T, pin: &str) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&gpio_sethigh(pin)).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_gpio_low<T: Read + Write>(port: &mut T, pin: &str) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&gpio_setlow(pin)).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_i2c_init<T: Read + Write>(
    port: &mut T,
    ident: &str,
    scl_pin: &str,
    sda_pin: &str,
    speed: u32,
) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&i2c_init(scl_pin, sda_pin, speed)).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_i2c_write<T: Read + Write>(
    port: &mut T,
    ident: &str,
    addr: u8,
    data: &[u8],
) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&i2c_write(ident, addr, data)).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_spi_init<T: Read + Write>(
    port: &mut T,
    ident: &str,
    sck_pin: &str,
    miso_pin: &str,
    mosi_pin: &str,
    speed: u32,
) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&spi_init(sck_pin, miso_pin, mosi_pin, speed)).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}

pub fn send_spi_write<T: Read + Write>(port: &mut T, ident: &str, data: &[u8]) -> io::Result<()> {
    let mut buf: Vec<u8, BufferLength> = (0..31).collect();
    let req: Vec<u8, BufferLength> = to_vec(&spi_write(ident, data)).unwrap();

    log::debug!(
        "Will send {} bytes containing {:?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    log::debug!("Received {:?} bytes containing {:?}", bytes, res);

    Ok(())
}
