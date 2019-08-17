#![no_main]
#![no_std]
#![allow(deprecated)]

use panic_halt as _;

use stm32f0xx_hal as hal;

use cortex_m_rt::entry;

use crate::hal::{i2c::I2c, prelude::*, serial::Serial, stm32};

use cortex_m::peripheral::Peripherals;
use nb::block;

use core::mem::transmute_copy;
use core::ops::Deref;

use heapless::{consts::*, Vec};
use postcard::{from_bytes, to_vec};

use bridge_common::encoding::{Reply, Request};

use stm32f0xx_hal::gpio::{gpioa, gpiob, gpiof};
use stm32f0xx_hal::gpio::{Floating, PushPull};
use stm32f0xx_hal::gpio::{Input, Output};

trait PORTExt {
    fn clone(&mut self) -> Self;
}

trait GPIOExt {
    fn to_output_push_pull(&self);
    fn toggle(&mut self);
    fn set_high(&mut self);
    fn set_low(&mut self);
}

macro_rules! PORT {
    ($port:ident) => {
        impl PORTExt for $port::Parts {
            fn clone(&mut self) -> Self {
                unsafe { transmute_copy(&self) }
            }
        }
    };
}

macro_rules! GPIO {
    ($port:ident, $pin:ident) => {
        impl GPIOExt for $port::$pin<Input<Floating>> {
            fn to_output_push_pull(&self) {
                cortex_m::interrupt::free(|cs| {
                    let pin: $port::$pin<Input<Floating>> = unsafe { transmute_copy(&self) };
                    pin.into_push_pull_output(cs);
                });
            }

            fn toggle(&mut self) {
                let mut pin: $port::$pin<Output<PushPull>> = unsafe { transmute_copy(&self) };
                pin.toggle();
            }

            fn set_high(&mut self) {
                let mut pin: $port::$pin<Output<PushPull>> = unsafe { transmute_copy(&self) };
                pin.set_high();
            }

            fn set_low(&mut self) {
                let mut pin: $port::$pin<Output<PushPull>> = unsafe { transmute_copy(&self) };
                pin.set_low();
            }
        }
    };
}

PORT!(gpioa);
PORT!(gpiob);
PORT!(gpiof);

GPIO!(gpioa, PA0);
GPIO!(gpioa, PA1);
GPIO!(gpioa, PA3);
GPIO!(gpioa, PA4);
GPIO!(gpioa, PA5);
GPIO!(gpioa, PA6);
GPIO!(gpioa, PA7);
GPIO!(gpioa, PA8);
GPIO!(gpioa, PA9);
GPIO!(gpioa, PA10);
GPIO!(gpioa, PA11);
GPIO!(gpioa, PA12);
GPIO!(gpioa, PA13);
GPIO!(gpioa, PA14);
GPIO!(gpiob, PB3);
GPIO!(gpiob, PB4);
GPIO!(gpiof, PF0);
GPIO!(gpiof, PF1);

fn send_serial_reply<T: embedded_hal::serial::Write<u8>>(serial: &mut T, reply: &Reply) {
    let output: Vec<u8, U32> = to_vec(reply).unwrap();
    for c in &output {
        serial.write(*c).ok();
    }
}

enum GPIO {
    A0,
    A1,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    A11,
    A12,
    A13,
    A14,
    B3,
    B4,
    F0,
    F1,
}

fn map_gpios(name: &str) -> Option<GPIO> {
    match name {
        "a0" => Some(GPIO::A0),
        "a1" => Some(GPIO::A1),
        "a3" => Some(GPIO::A3),
        "a4" => Some(GPIO::A4),
        "a5" => Some(GPIO::A5),
        "a6" => Some(GPIO::A6),
        "a7" => Some(GPIO::A7),
        "a8" => Some(GPIO::A8),
        "a9" => Some(GPIO::A9),
        "a10" => Some(GPIO::A10),
        "a11" => Some(GPIO::A11),
        "a12" => Some(GPIO::A12),
        "a13" => Some(GPIO::A13),
        "a14" => Some(GPIO::A14),
        "b3" => Some(GPIO::B3),
        "b4" => Some(GPIO::B4),
        "f0" => Some(GPIO::F0),
        "f1" => Some(GPIO::F1),

        _ => None,
    }
}

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(_cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);

        // Obtain resources from GPIO ports A, B and F
        let mut gpioa = p.GPIOA.split(&mut rcc);
        let mut gpiob = p.GPIOB.split(&mut rcc);
        let mut gpiof = p.GPIOF.split(&mut rcc);

        let mut i2c: Option<hal::i2c::I2c<_, _, _>> = None;

        /* Set up serial port */
        let (tx, rx) = cortex_m::interrupt::free(|cs| {
            let gpioa = gpioa.clone();
            let pa2 = gpioa.pa2;
            let pa15 = gpioa.pa15;
            (pa2.into_alternate_af1(cs), pa15.into_alternate_af1(cs))
        });

        let mut serial = Serial::usart2(p.USART2, (tx, rx), 115_200.bps(), &mut rcc);

        let mut buffer: Vec<u8, U32> = Default::default();

        loop {
            if let Ok(received) = block!(serial.read()) {
                if buffer.push(received).is_err() {
                    send_serial_reply(
                        &mut serial,
                        &Reply::ReceiveErr {
                            bytes: buffer.len() as u8,
                        },
                    );
                    buffer.clear();
                } else {
                    match from_bytes::<Request>(buffer.deref()) {
                        Ok(msg) => {
                            if let Request::GpioInitPP { pin } = msg {
                                if let Some(pin) = map_gpios(pin) {
                                    match pin {
                                        GPIO::A0 => gpioa.pa0.to_output_push_pull(),
                                        GPIO::A1 => gpioa.pa1.to_output_push_pull(),
                                        GPIO::A3 => gpioa.pa3.to_output_push_pull(),
                                        GPIO::A4 => gpioa.pa4.to_output_push_pull(),
                                        GPIO::A5 => gpioa.pa5.to_output_push_pull(),
                                        GPIO::A6 => gpioa.pa6.to_output_push_pull(),
                                        GPIO::A7 => gpioa.pa7.to_output_push_pull(),
                                        GPIO::A8 => gpioa.pa8.to_output_push_pull(),
                                        GPIO::A9 => gpioa.pa9.to_output_push_pull(),
                                        GPIO::A10 => gpioa.pa10.to_output_push_pull(),
                                        GPIO::A11 => gpioa.pa11.to_output_push_pull(),
                                        GPIO::A12 => gpioa.pa12.to_output_push_pull(),
                                        GPIO::A13 => gpioa.pa13.to_output_push_pull(),
                                        GPIO::A14 => gpioa.pa14.to_output_push_pull(),
                                        GPIO::B3 => gpiob.pb3.to_output_push_pull(),
                                        GPIO::B4 => gpiob.pb4.to_output_push_pull(),
                                        GPIO::F0 => gpiof.pf0.to_output_push_pull(),
                                        GPIO::F1 => gpiof.pf1.to_output_push_pull(),
                                    }
                                    send_serial_reply(&mut serial, &Reply::Ok {});
                                } else {
                                    send_serial_reply(&mut serial, &Reply::NotImplemented {});
                                }
                            } else if let Request::GpioToggle { pin } = msg {
                                if let Some(pin) = map_gpios(pin) {
                                    match pin {
                                        GPIO::A0 => gpioa.pa0.toggle(),
                                        GPIO::A1 => gpioa.pa1.toggle(),
                                        GPIO::A3 => gpioa.pa3.toggle(),
                                        GPIO::A4 => gpioa.pa4.toggle(),
                                        GPIO::A5 => gpioa.pa5.toggle(),
                                        GPIO::A6 => gpioa.pa6.toggle(),
                                        GPIO::A7 => gpioa.pa7.toggle(),
                                        GPIO::A8 => gpioa.pa8.toggle(),
                                        GPIO::A9 => gpioa.pa9.toggle(),
                                        GPIO::A10 => gpioa.pa10.toggle(),
                                        GPIO::A11 => gpioa.pa11.toggle(),
                                        GPIO::A12 => gpioa.pa12.toggle(),
                                        GPIO::A13 => gpioa.pa13.toggle(),
                                        GPIO::A14 => gpioa.pa14.toggle(),
                                        GPIO::B3 => gpiob.pb3.toggle(),
                                        GPIO::B4 => gpiob.pb4.toggle(),
                                        GPIO::F0 => gpiof.pf0.toggle(),
                                        GPIO::F1 => gpiof.pf1.toggle(),
                                    }
                                    send_serial_reply(&mut serial, &Reply::Ok {});
                                } else {
                                    send_serial_reply(&mut serial, &Reply::NotImplemented {});
                                }
                            } else if let Request::GpioSetLow { pin } = msg {
                                if let Some(pin) = map_gpios(pin) {
                                    match pin {
                                        GPIO::A0 => gpioa.pa0.set_low(),
                                        GPIO::A1 => gpioa.pa1.set_low(),
                                        GPIO::A3 => gpioa.pa3.set_low(),
                                        GPIO::A4 => gpioa.pa4.set_low(),
                                        GPIO::A5 => gpioa.pa5.set_low(),
                                        GPIO::A6 => gpioa.pa6.set_low(),
                                        GPIO::A7 => gpioa.pa7.set_low(),
                                        GPIO::A8 => gpioa.pa8.set_low(),
                                        GPIO::A9 => gpioa.pa9.set_low(),
                                        GPIO::A10 => gpioa.pa10.set_low(),
                                        GPIO::A11 => gpioa.pa11.set_low(),
                                        GPIO::A12 => gpioa.pa12.set_low(),
                                        GPIO::A13 => gpioa.pa13.set_low(),
                                        GPIO::A14 => gpioa.pa14.set_low(),
                                        GPIO::B3 => gpiob.pb3.set_low(),
                                        GPIO::B4 => gpiob.pb4.set_low(),
                                        GPIO::F0 => gpiof.pf0.set_low(),
                                        GPIO::F1 => gpiof.pf1.set_low(),
                                    }
                                    send_serial_reply(&mut serial, &Reply::Ok {});
                                } else {
                                    send_serial_reply(&mut serial, &Reply::NotImplemented {});
                                }
                            } else if let Request::GpioSetHigh { pin } = msg {
                                if let Some(pin) = map_gpios(pin) {
                                    match pin {
                                        GPIO::A0 => gpioa.pa0.set_high(),
                                        GPIO::A1 => gpioa.pa1.set_high(),
                                        GPIO::A3 => gpioa.pa3.set_high(),
                                        GPIO::A4 => gpioa.pa4.set_high(),
                                        GPIO::A5 => gpioa.pa5.set_high(),
                                        GPIO::A6 => gpioa.pa6.set_high(),
                                        GPIO::A7 => gpioa.pa7.set_high(),
                                        GPIO::A8 => gpioa.pa8.set_high(),
                                        GPIO::A9 => gpioa.pa9.set_high(),
                                        GPIO::A10 => gpioa.pa10.set_high(),
                                        GPIO::A11 => gpioa.pa11.set_high(),
                                        GPIO::A12 => gpioa.pa12.set_high(),
                                        GPIO::A13 => gpioa.pa13.set_high(),
                                        GPIO::A14 => gpioa.pa14.set_high(),
                                        GPIO::B3 => gpiob.pb3.set_high(),
                                        GPIO::B4 => gpiob.pb4.set_high(),
                                        GPIO::F0 => gpiof.pf0.set_high(),
                                        GPIO::F1 => gpiof.pf1.set_high(),
                                    }
                                    send_serial_reply(&mut serial, &Reply::Ok {});
                                } else {
                                    send_serial_reply(&mut serial, &Reply::NotImplemented {});
                                }
                            } else if let Request::I2CInit {
                                scl_pin,
                                sda_pin,
                                speed,
                            } = msg
                            {
                                if scl_pin == "f1"
                                    && sda_pin == "f0"
                                    && (speed >= 10 || speed <= 400)
                                {
                                    let (scl, sda) = cortex_m::interrupt::free(|cs| {
                                        let gpiof = gpiof.clone();
                                        let scl = gpiof
                                            .pf1
                                            .into_alternate_af1(cs)
                                            .internal_pull_up(cs, true)
                                            .set_open_drain(cs);
                                        let sda = gpiof
                                            .pf0
                                            .into_alternate_af1(cs)
                                            .internal_pull_up(cs, true)
                                            .set_open_drain(cs);
                                        (scl, sda)
                                    });

                                    let i2c1 = unsafe { transmute_copy(&p.I2C1) };

                                    // Setup I2C1
                                    i2c = Some(I2c::i2c1(i2c1, (scl, sda), speed.khz(), &mut rcc));

                                    send_serial_reply(&mut serial, &Reply::Ok {});
                                } else {
                                    send_serial_reply(&mut serial, &Reply::NotImplemented {});
                                }
                            } else if let Request::I2CWrite {
                                ident,
                                address,
                                data,
                            } = msg
                            {
                                if ident == "i2c1" && i2c.is_some() {
                                    i2c.as_mut().map(|i2c| i2c.write(address, data));
                                    send_serial_reply(&mut serial, &Reply::Ok {});
                                } else {
                                    send_serial_reply(&mut serial, &Reply::NotImplemented {});
                                }
                            } else {
                                send_serial_reply(&mut serial, &Reply::NotImplemented {});
                            }
                            buffer.clear();
                        }
                        Err(err) => match err {
                            postcard::Error::DeserializeUnexpectedEnd => {}
                            _ => {
                                send_serial_reply(
                                    &mut serial,
                                    &Reply::VerboseErr { err: "some error" },
                                );
                            }
                        },
                    }
                }
            } else {
                serial.flush().ok();
            }
        }
    }

    loop {
        continue;
    }
}
