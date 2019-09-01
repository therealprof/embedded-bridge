use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crate::io::{send_clear, send_version};

pub fn assert_version<T: Read + Write>(channel: Arc<Mutex<Box<T>>>) {
    send_clear(&mut *channel.lock().unwrap()).ok();

    let target_version =
        send_version(&mut *channel.lock().unwrap()).expect("Could not get target version");

    if target_version != bridge_common::encoding::VERSION {
        panic!(
            "Protocol versions locally and on the target differ, there: {}, here: {}",
            target_version,
            bridge_common::encoding::VERSION
        );
    } else {
        log::info!(
            "Bridge protocol version {}",
            bridge_common::encoding::VERSION
        );
    }
}
