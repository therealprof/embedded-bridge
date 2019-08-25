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

#[cfg(any(feature = "stm32f072",))]
use stm32f0xx_hal::gpio::gpioc;

use stm32f0xx_hal::gpio::{Floating, PushPull};
use stm32f0xx_hal::gpio::{Input, Output};

trait PORTExt {
    fn clone(&self) -> Self;
}

trait GPIOExt {
    fn to_output_push_pull(&self);
    fn toggle(&self);
    fn set_high(&self);
    fn set_low(&self);
}

macro_rules! PORT {
    ($port:ident) => {
        impl PORTExt for $port::Parts {
            fn clone(&self) -> Self {
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

            fn toggle(&self) {
                let mut pin: $port::$pin<Output<PushPull>> = unsafe { transmute_copy(&self) };
                pin.toggle();
            }

            fn set_high(&self) {
                let mut pin: $port::$pin<Output<PushPull>> = unsafe { transmute_copy(&self) };
                pin.set_high();
            }

            fn set_low(&self) {
                let mut pin: $port::$pin<Output<PushPull>> = unsafe { transmute_copy(&self) };
                pin.set_low();
            }
        }
    };
}

PORT!(gpioa);
PORT!(gpiob);
#[cfg(any(feature = "stm32f072",))]
PORT!(gpioc);
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
#[cfg(any(feature = "stm32f072",))]
GPIO!(gpioc, PC6);
#[cfg(any(feature = "stm32f072",))]
GPIO!(gpioc, PC7);
#[cfg(any(feature = "stm32f072",))]
GPIO!(gpioc, PC8);
#[cfg(any(feature = "stm32f072",))]
GPIO!(gpioc, PC9);

fn send_serial_reply<T: embedded_hal::serial::Write<u8>>(serial: &mut T, reply: &Reply) {
    let output: Vec<u8, U32> = to_vec(reply).unwrap();
    for c in &output {
        serial.write(*c).ok();
    }
    serial.flush().ok();
}

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(_cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut p.FLASH);

        // Obtain resources from GPIO ports A, B, C and F
        let gpioa = p.GPIOA.split(&mut rcc);
        let gpiob = p.GPIOB.split(&mut rcc);
        #[cfg(any(feature = "stm32f072",))]
        let gpioc = p.GPIOC.split(&mut rcc);
        let gpiof = p.GPIOF.split(&mut rcc);

        #[cfg(any(feature = "stm32f042",))]
        const HAS_I2C_ON_PORT_F: bool = true;
        #[cfg(not(feature = "stm32f042",))]
        const HAS_I2C_ON_PORT_F: bool = false;

        #[cfg(any(feature = "stm32f042",))]
        let mut i2c: Option<hal::i2c::I2c<_, _, _>> = None;

        #[cfg(not(feature = "stm32f042",))]
        let i2c: Option<bool> = None;

        /* Set up serial port */
        let (tx, rx) = cortex_m::interrupt::free(|cs| {
            let gpioa = gpioa.clone();
            let pa2 = gpioa.pa2;
            let pa15 = gpioa.pa15;
            (pa2.into_alternate_af1(cs), pa15.into_alternate_af1(cs))
        });

        let mut serial = Serial::usart2(p.USART2, (tx, rx), 115_200.bps(), &mut rcc);

        let mut buffer: Vec<u8, U32> = Default::default();

        let map_gpio = |name: &str| -> Option<&dyn GPIOExt> {
            match name {
                "a0" => Some(&gpioa.pa0),
                "a1" => Some(&gpioa.pa1),
                "a3" => Some(&gpioa.pa3),
                "a4" => Some(&gpioa.pa4),
                "a5" => Some(&gpioa.pa5),
                "a6" => Some(&gpioa.pa6),
                "a7" => Some(&gpioa.pa7),
                "a8" => Some(&gpioa.pa8),
                "a9" => Some(&gpioa.pa9),
                "a10" => Some(&gpioa.pa10),
                "a11" => Some(&gpioa.pa11),
                "a12" => Some(&gpioa.pa12),
                "a13" => Some(&gpioa.pa13),
                "a14" => Some(&gpioa.pa14),
                "b3" => Some(&gpiob.pb3),
                "b4" => Some(&gpiob.pb4),
                #[cfg(any(feature = "stm32f072"))]
                "c6" => Some(&gpioc.pc6),
                #[cfg(any(feature = "stm32f072"))]
                "c7" => Some(&gpioc.pc7),
                #[cfg(any(feature = "stm32f072"))]
                "c8" => Some(&gpioc.pc8),
                #[cfg(any(feature = "stm32f072"))]
                "c9" => Some(&gpioc.pc9),
                "f0" => Some(&gpiof.pf0),
                "f1" => Some(&gpiof.pf1),
                _ => None,
            }
        };

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
                    continue;
                }
            }

            match from_bytes::<Request>(buffer.deref()) {
                Ok(msg) => {
                    if let Request::GpioInitPP { pin } = msg {
                        if let Some(p) = map_gpio(pin) {
                            p.to_output_push_pull();
                            send_serial_reply(&mut serial, &Reply::Ok {});
                        } else {
                            send_serial_reply(&mut serial, &Reply::NotImplemented {});
                        }
                    } else if let Request::GpioToggle { pin } = msg {
                        if let Some(p) = map_gpio(pin) {
                            p.toggle();
                            send_serial_reply(&mut serial, &Reply::Ok {});
                        } else {
                            send_serial_reply(&mut serial, &Reply::NotImplemented {});
                        }
                    } else if let Request::GpioSetLow { pin } = msg {
                        if let Some(p) = map_gpio(pin) {
                            p.set_low();
                            send_serial_reply(&mut serial, &Reply::Ok {});
                        } else {
                            send_serial_reply(&mut serial, &Reply::NotImplemented {});
                        }
                    } else if let Request::GpioSetHigh { pin } = msg {
                        if let Some(p) = map_gpio(pin) {
                            p.set_high();
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
                        if HAS_I2C_ON_PORT_F
                            && scl_pin == "f1"
                            && sda_pin == "f0"
                            && (speed >= 10 || speed <= 400)
                        {
                            #[cfg(any(feature = "stm32f042",))]
                            {
                                let gpiof = gpiof.clone();
                                let (scl, sda) = cortex_m::interrupt::free(|cs| {
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
                            }
                        } else {
                            send_serial_reply(&mut serial, &Reply::NotImplemented {});
                        }
                    } else if let Request::I2CWrite {
                        ident,
                        address,
                        data,
                    } = msg
                    {
                        if HAS_I2C_ON_PORT_F && ident == "i2c1" && i2c.is_some() {
                            #[cfg(any(feature = "stm32f042",))]
                            {
                                i2c.as_mut().map(|i2c| i2c.write(address, data));
                                send_serial_reply(&mut serial, &Reply::Ok {});
                            }
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
                        send_serial_reply(&mut serial, &Reply::VerboseErr { err: "some error" });
                    }
                },
            }
        }
    }

    loop {
        continue;
    }
}
