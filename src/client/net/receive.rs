// SPDX-License-Identifier: GPL-3.0-only
//! Receiver Thread
//!
//! Authors: MarioS271

use crate::client::ui::state::ClientState;
use crate::client::ui::tui::end_raw_mode;
use crate::framing;
use crate::message::Message;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

const LOG_PREFIX: &str = "(receive thread)";

pub fn receive_thread(mut read_stream: TcpStream, state_recv: Arc<Mutex<ClientState>>) {
    loop {
        match framing::read_message(&mut read_stream) {
            Ok(data) => {
                match Message::deserialize(data.as_slice()) {
                    Ok(msg) => state_recv.lock().unwrap().messages.push(msg),
                    Err(err) => {
                        end_raw_mode();
                        eprintln!("{} Deserialize Error: {}", LOG_PREFIX, err);
                    }
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                end_raw_mode();
                eprintln!("{} Disconnected", LOG_PREFIX);
                std::process::exit(1);
            }
            Err(err) => {
                end_raw_mode();
                eprintln!("{} Receive Error: {}", LOG_PREFIX, err);
                std::process::exit(1);
            }
        }
    }
}
