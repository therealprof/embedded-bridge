use std::env;
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serial::prelude::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use embedded_hal::digital::v1::OutputPin;

use std::collections::HashMap;

fn usage() {
    println!("Overview of implemented commands:");
    println!("  gpio: Control individual IO pins");
    println!("    init <pin>: Initialiase remote GPIO pin identified by <pin> into push pull mode");
    println!("    set <pin> (low|high): Set the signal level of the remote GPIO pin identified by <pin> low or high");
    println!("  help: This help");
    println!("  quit (or exit): Exit this tool");
}

fn main() -> io::Result<()> {
    println!("Welcome to the embedded-bridge CLI tool");

    let serial = &env::args_os().nth(1).unwrap();

    println!("Connecting to the device via (USB<->)serial port at {:?}", &serial);

    let mut port = serial::open(&serial).unwrap();
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

    println!("Checking compatible firmware version...");

    bridge_host::common::assert_version(port.clone());

    let mut rl = Editor::<()>::new();

    rl.load_history("history.txt").ok();

    let name = "embedded-bridge";

    let mut gpios: HashMap<String, bridge_host::gpio::PushPullPin<serial::SystemPort>> =
        HashMap::new();

    loop {
        let prompt = format!("{} >> ", name);
        match rl.readline(&prompt) {
            Ok(line) => {
                rl.add_history_entry(&line);
                match line.split_whitespace().collect::<Vec<&str>>().split_first() {
                    Some((&"gpio", rest)) => match rest.len() {
                        2 => match rest[0] {
                            "init" => {
                                let pin = bridge_host::gpio::PushPullPin::new(
                                    rest[1].to_string(),
                                    port.clone(),
                                );
                                gpios.insert(rest[1].to_string(), pin);
                            }
                            _ => println!("Expecting arguments"),
                        },
                        3 => match rest[0] {
                            "set" => {
                                if let Some(ref mut pin) = gpios.get_mut(&rest[1].to_string()) {
                                    match rest[2] {
                                        "low" | "off" => pin.set_low(),
                                        "high" | "on" => pin.set_high(),
                                        _ => println!("Expecting low or high as signal state"),
                                    }
                                } else {
                                    println!("No initialised GPIO {}", &rest[1].to_string());
                                }
                            }
                            _ => println!("Expecting arguments"),
                        },
                        4..=1000 => println!("Too many arguments for 'gpio'"),
                        _ => println!("Too few arguments for 'gpio'"),
                    },
                    Some((&"exit", _)) | Some((&"quit", _)) => break,
                    Some((&"help", _)) | Some((&"h", _)) => usage(),
                    Some((&s, _)) => println!("Don't know what '{}' is, try 'h' for help", s),
                    None => println!("What is it you'd like to say? Try 'h' for help"),
                }
            }
            Err(e) => println!("Something weird happened {}", e),
        }
    }

    rl.save_history("history.txt").unwrap();

    Ok(())
}
