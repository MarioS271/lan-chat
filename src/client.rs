// SPDX-License-Identifier: GPL-3.0-only
//! Client Mode Logic
//!
//! Authors: MarioS271

use std::net::TcpStream;

pub fn run_client(address: &str) -> std::io::Result<()> {
    println!("Running as client and connecting to {}\n", address);

    let stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;

    println!("Connected to {}", stream.peer_addr()?);

    let mut read_stream = stream.try_clone()?;
    let mut write_stream = stream;

    std::thread::spawn(move || {
        receive_thread(read_stream);
    });

    main_thread(write_stream);
    Ok(())
}

fn main_thread(mut stream: TcpStream) {
    let stdin = std::io::stdin();
    let mut line = String::new();

    loop {
        line.clear();

        if stdin.read_line(&mut line).is_err() || line.is_empty() {
            break;
        }

        let data = line.trim_end().as_bytes();
        if let Err(err) = crate::framing::write_message(&mut stream, data) {
            eprintln!("Write Error: {}", err);
            break;
        }
    }
}

fn receive_thread(mut stream: TcpStream) {
    loop {
        match crate::framing::read_message(&mut stream) {
            Ok(data) => {
                let text = String::from_utf8_lossy(&data);
                println!("{}", text);
            },
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("Server disconnected");
                break;
            },
            Err(err) => {
                eprintln!("Read Error: {}", err);
                break;
            }
        }
    }
}
