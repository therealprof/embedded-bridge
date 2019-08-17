use bridge_common::encoding::{
    gpio_init_pp, gpio_sethigh, gpio_setlow, gpio_toggle, i2c_init, i2c_write, Reply, Request,
};
use heapless::{consts::*, Vec};
use postcard::{from_bytes, to_vec};
use std::io::{self, Read, Write};
use std::ops::Deref;

pub fn send_gpio_init_pp<T: Read + Write>(port: &mut T, pin: &str) -> io::Result<()> {
    let mut buf: Vec<u8, U32> = (0..31).collect();
    let req: Vec<u8, U32> = to_vec(&gpio_init_pp(pin)).unwrap();

    println!(
        "Will send {} {:#?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    println!("Reply {:#?} {:#?}", bytes, res);

    Ok(())
}

pub fn send_gpio_toggle<T: Read + Write>(port: &mut T, pin: &str) -> io::Result<()> {
    let mut buf: Vec<u8, U32> = (0..31).collect();
    let req: Vec<u8, U32> = to_vec(&gpio_toggle(pin)).unwrap();

    println!(
        "Will send {} {:#?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    println!("Reply {:#?} {:#?}", bytes, res);

    Ok(())
}

pub fn send_gpio_high<T: Read + Write>(port: &mut T, pin: &str) -> io::Result<()> {
    let mut buf: Vec<u8, U32> = (0..31).collect();
    let req: Vec<u8, U32> = to_vec(&gpio_sethigh(pin)).unwrap();

    println!(
        "Will send {} {:#?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    println!("Reply {:#?} {:#?}", bytes, res);

    Ok(())
}

pub fn send_gpio_low<T: Read + Write>(port: &mut T, pin: &str) -> io::Result<()> {
    let mut buf: Vec<u8, U32> = (0..31).collect();
    let req: Vec<u8, U32> = to_vec(&gpio_setlow(pin)).unwrap();

    println!(
        "Will send {} {:#?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    println!("Reply {:#?} {:#?}", bytes, res);

    Ok(())
}

pub fn send_i2c_init<T: Read + Write>(port: &mut T, ident: &str, scl_pin: &str, sda_pin: &str, speed: u32) -> io::Result<()> {
    let mut buf: Vec<u8, U32> = (0..31).collect();
    let req: Vec<u8, U32> = to_vec(&i2c_init(scl_pin, sda_pin, speed)).unwrap();

    println!(
        "Will send {} {:#?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    println!("Reply {:#?} {:#?}", bytes, res);

    Ok(())
}


pub fn send_i2c_write<T: Read + Write>(port: &mut T, ident: &str, addr: u8, data: &[u8]) -> io::Result<()> {
    let mut buf: Vec<u8, U32> = (0..31).collect();
    let req: Vec<u8, U32> = to_vec(&i2c_write(ident, addr, data)).unwrap();

    println!(
        "Will send {} {:#?}",
        req.len(),
        from_bytes::<Request>(req.deref()).unwrap()
    );

    port.write_all(&req)?;
    let bytes = port.read(&mut buf[..])?;
    let res = from_bytes::<Reply>(buf.deref());

    println!("Reply {:#?} {:#?}", bytes, res);

    Ok(())
}

