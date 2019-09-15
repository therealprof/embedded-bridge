#![no_main]
#![no_std]
#![allow(deprecated)]

use panic_halt as _;

use stm32f0xx_hal as hal;

use cortex_m_rt::entry;

use crate::hal::{i2c::I2c, prelude::*, serial::Serial, spi::Spi, stm32};

use cortex_m::peripheral::Peripherals;
use nb::block;

use core::mem::transmute_copy;

use heapless::{consts::*, Vec};
use postcard::{from_bytes, to_vec};

use bridge_common::encoding::{Reply, Request};

use stm32f0xx_hal::gpio::{gpioa, gpiob, gpiof};

#[cfg(any(feature = "stm32f072",))]
use stm32f0xx_hal::gpio::gpioc;

use stm32f0xx_hal::gpio::{Floating, PushPull};
use stm32f0xx_hal::gpio::{Input, Output};

type BufferLength = U64;

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
    ($port:ident, $pin:ident, $name:ident) => {
        struct $pin($port::$pin<Input<Floating>>);

        let $name = $pin($port.$name);

        impl GPIOExt for $pin {
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

fn send_serial_reply<T: embedded_hal::serial::Write<u8>>(serial: &mut T, reply: &Reply) {
    let output: Vec<u8, BufferLength> = to_vec(reply).unwrap();
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

        PORT!(gpioa);
        PORT!(gpiob);
        #[cfg(any(feature = "stm32f072",))]
        PORT!(gpioc);
        PORT!(gpiof);

        #[cfg(any(feature = "stm32f042",))]
        const HAS_I2C_ON_PORT_F: bool = true;
        #[cfg(not(feature = "stm32f042",))]
        const HAS_I2C_ON_PORT_F: bool = false;

        #[cfg(any(feature = "stm32f042",))]
        let mut i2c: Option<hal::i2c::I2c<_, _, _>> = None;

        let mut spi: Option<hal::spi::Spi<_, _, _, _>> = None;

        #[cfg(not(feature = "stm32f042",))]
        let i2c: Option<bool> = None;

        /* Set up serial port */
        let (tx, rx) = cortex_m::interrupt::free(|cs| {
            let gpioa = gpioa.clone();
            let pa2 = gpioa.pa2;
            let pa15 = gpioa.pa15;
            (pa2.into_alternate_af1(cs), pa15.into_alternate_af1(cs))
        });

        let gpioa_clone = gpioa.clone();
        let gpiof_clone = gpiof.clone();

        let mut serial = Serial::usart2(p.USART2, (tx, rx), 115_200.bps(), &mut rcc);

        let mut buffer: Vec<u8, BufferLength> = Default::default();

        GPIO!(gpioa, PA0, pa0);
        GPIO!(gpioa, PA1, pa1);
        GPIO!(gpioa, PA3, pa3);
        GPIO!(gpioa, PA4, pa4);
        GPIO!(gpioa, PA5, pa5);
        GPIO!(gpioa, PA6, pa6);
        GPIO!(gpioa, PA7, pa7);
        GPIO!(gpioa, PA8, pa8);
        GPIO!(gpioa, PA9, pa9);
        GPIO!(gpioa, PA10, pa10);
        GPIO!(gpioa, PA11, pa11);
        GPIO!(gpioa, PA12, pa12);
        GPIO!(gpioa, PA13, pa13);
        GPIO!(gpioa, PA14, pa14);
        GPIO!(gpiob, PB3, pb3);
        GPIO!(gpiob, PB4, pb4);
        GPIO!(gpiof, PF0, pf0);
        GPIO!(gpiof, PF1, pf1);
        #[cfg(any(feature = "stm32f072",))]
        GPIO!(gpioc, PC6, pc6);
        #[cfg(any(feature = "stm32f072",))]
        GPIO!(gpioc, PC7, pc7);
        #[cfg(any(feature = "stm32f072",))]
        GPIO!(gpioc, PC8, pc8);
        #[cfg(any(feature = "stm32f072",))]
        GPIO!(gpioc, PC9, pc9);

        let apply_gpio = |name: &str, f: &dyn Fn(&dyn GPIOExt)| -> Reply {
            match name {
                "a0" => f(&pa0),
                "a1" => f(&pa1),
                "a3" => f(&pa3),
                "a4" => f(&pa4),
                "a5" => f(&pa5),
                "a6" => f(&pa6),
                "a7" => f(&pa7),
                "a8" => f(&pa8),
                "a9" => f(&pa9),
                "a10" => f(&pa10),
                "a11" => f(&pa11),
                "a12" => f(&pa12),
                "a13" => f(&pa13),
                "a14" => f(&pa14),
                "b3" => f(&pb3),
                "b4" => f(&pb4),
                #[cfg(any(feature = "stm32f072"))]
                "c6" => f(&pc6),
                #[cfg(any(feature = "stm32f072"))]
                "c7" => f(&pc7),
                #[cfg(any(feature = "stm32f072"))]
                "c8" => f(&pc8),
                #[cfg(any(feature = "stm32f072"))]
                "c9" => f(&pc9),
                "f0" => f(&pf0),
                "f1" => f(&pf1),
                _ => return Reply::NotImplemented,
            }
            Reply::Ok
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

            let reply = match from_bytes::<Request>(&buffer) {
                Ok(msg) => {
                    match msg {
                        Request::Version => bridge_common::encoding::current_version(),
                        Request::Clear => Reply::Ok {},
                        Request::Reset => Reply::NotImplemented {},
                        Request::GpioInitPP { pin } => {
                            apply_gpio(pin, &|p: &dyn GPIOExt| p.to_output_push_pull())
                        }

                        Request::GpioToggle { pin } => {
                            apply_gpio(pin, &|p: &dyn GPIOExt| p.toggle())
                        }

                        Request::GpioSetLow { pin } => {
                            apply_gpio(pin, &|p: &dyn GPIOExt| p.set_low())
                        }

                        Request::GpioSetHigh { pin } => {
                            apply_gpio(pin, &|p: &dyn GPIOExt| p.set_high())
                        }

                        Request::I2CInit {
                            scl_pin,
                            sda_pin,
                            speed,
                        } => {
                            if HAS_I2C_ON_PORT_F
                                && scl_pin == "f1"
                                && sda_pin == "f0"
                                && (speed >= 10 || speed <= 400)
                            {
                                #[cfg(any(feature = "stm32f042",))]
                                {
                                    let gpiof = gpiof_clone.clone();
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
                                }

                                Reply::Ok {}
                            } else {
                                Reply::NotImplemented {}
                            }
                        }

                        Request::I2CWrite {
                            ident,
                            address,
                            data,
                        } => {
                            if HAS_I2C_ON_PORT_F && ident == "i2c1" && i2c.is_some() {
                                #[cfg(any(feature = "stm32f042",))]
                                {
                                    i2c.as_mut().map(|i2c| i2c.write(address, data));
                                }

                                Reply::Ok {}
                            } else {
                                Reply::NotImplemented {}
                            }
                        }

                        Request::SPIInit {
                            sck_pin,
                            miso_pin,
                            mosi_pin,
                            speed,
                        } => {
                            if sck_pin == "a5"
                                && miso_pin == "a6"
                                && mosi_pin == "a7"
                                && (speed >= 10 || speed <= 400)
                            {
                                let gpioa = gpioa_clone.clone();
                                let (sck, miso, mosi) = cortex_m::interrupt::free(move |cs| {
                                    (
                                        gpioa.pa5.into_alternate_af0(cs),
                                        gpioa.pa6.into_alternate_af0(cs),
                                        gpioa.pa7.into_alternate_af0(cs),
                                    )
                                });

                                let spi1 = unsafe { transmute_copy(&p.SPI1) };

                                use embedded_hal::spi::{Mode, Phase, Polarity};

                                /// SPI mode that is needed for this crate
                                ///
                                /// Provided for convenience
                                const MODE: Mode = Mode {
                                    polarity: Polarity::IdleHigh,
                                    phase: Phase::CaptureOnSecondTransition,
                                };

                                // Setup SPI1
                                spi = Some(Spi::spi1(
                                    spi1,
                                    (sck, miso, mosi),
                                    MODE,
                                    speed.khz(),
                                    &mut rcc,
                                ));

                                Reply::Ok {}
                            } else {
                                Reply::NotImplemented {}
                            }
                        }

                        Request::SPIWrite { ident, data } => {
                            if ident == "spi1" && spi.is_some() {
                                spi.as_mut().map(|spi| spi.write(data));
                                Reply::Ok {}
                            } else {
                                Reply::NotImplemented {}
                            }
                        }
                    }
                }
                Err(err) => match err {
                    postcard::Error::DeserializeUnexpectedEnd => continue,
                    _ => Reply::VerboseErr { err: "some error" },
                },
            };

            /* Clear the buffer after parsing a complete message */
            buffer.clear();

            /* Send reply over serial connection */
            send_serial_reply(&mut serial, &reply);
        }
    }

    loop {
        continue;
    }
}
