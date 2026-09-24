// SPDX-License-Identifier: GPL-3.0-only
//! Client Mode Logic
//!
//! Authors: MarioS271

use crate::types::message::Message;
use crate::types::session_info::SessionInfo;
use std::net::TcpStream;

pub fn run_client(address: &str) -> std::io::Result<()> {
    let mut session_info = SessionInfo::default();
    session_info.read_in_name()?;

    println!("Running as client");
    println!("Attempting to connect to {}", address);

    let stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;

    println!("Successfully connected to {}", stream.peer_addr()?);

    session_info.read_in_ip(&stream)?;

    let read_stream = stream.try_clone()?;
    let write_stream = stream;

    std::thread::spawn(move || {
        receive_thread(read_stream);
    });

    main_thread(session_info, write_stream);
    Ok(())
}

fn main_thread(session_info: SessionInfo, mut stream: TcpStream) {
    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        input.clear();

        match stdin.read_line(&mut input) {
            Ok(0) => {
                eprintln!("\nIllegal Input, aborting");
                std::process::exit(1);
            },
            Err(err) => {
                eprintln!("\nInput Read Error: {}", err);
                std::process::exit(1);
            },
            _ => {}
        }

        if input.trim().is_empty() {
            continue;
        }

        let data = Message::new(&session_info, &input);

        if let Err(err) = crate::framing::write_message(&mut stream, data.serialize().as_slice()) {
            eprintln!("\nWrite Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn receive_thread(mut stream: TcpStream) {
    loop {
        match crate::framing::read_message(&mut stream) {
            Ok(data) => {
                let msg = match Message::deserialize(data.as_slice()) {
                    Ok(msg) => msg,
                    Err(err) => {
                        eprintln!("\nError decoding message: {}", err);
                        std::process::exit(1);
                    }
                };

                let name = std::str::from_utf8(&msg.sender_name)
                    .unwrap_or("?")
                    .trim_end_matches('\0');
                let content = String::from_utf8_lossy(&msg.content);

                println!("[{}] {}: {}", msg.timestamp_secs, name, content.trim());
            },
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("\nDisconnected");
                std::process::exit(0);
            },
            Err(err) => {
                eprintln!("\nReceive Read Error: {}", err);
                std::process::exit(1);
            }
        }
    }
}
