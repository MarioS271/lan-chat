// SPDX-License-Identifier: GPL-3.0-only
//! Server Mode Logic
//!
//! Authors: MarioS271

use crate::types::message::{Message, SystemMessage};
use crate::types::session_info::SessionInfo;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

// TODO: kick zombie clients

type ClientList = Arc<Mutex<Vec<TcpStream>>>;

pub fn run_server(port: u16) -> std::io::Result<()> {
    println!("Running as server on port {}", port);

    let listener = TcpListener::bind(("0.0.0.0", port))?;
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Connected: {}", stream.peer_addr()?);

        let clients = Arc::clone(&clients);
        std::thread::spawn(move || -> std::io::Result<()> {
            handle_client(stream, clients)
        });
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, clients: ClientList) -> std::io::Result<()> {
    stream.write_all(&crate::PROTOCOL_VERSION.to_be_bytes())?;

    let mut client_version = [0u8; 2];
    stream.read_exact(&mut client_version)?;

    if u16::from_be_bytes(client_version) != crate::PROTOCOL_VERSION {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Client has wrong protocol version"
        ));
    }

    let mut client_name = [0u8; SessionInfo::MAX_NAME_LEN];
    stream.read_exact(&mut client_name)?;

    let peer_addr = stream.peer_addr()?;
    let write_stream = stream.try_clone()?;
    clients.lock().unwrap().push(write_stream);

    let msg = Message::System(SystemMessage::new(format!(
        "{} ({}) connected",
        std::str::from_utf8(&client_name)
            .unwrap_or("\u{FFFD}")
            .trim_end_matches('\0')
            .trim_end(),
        peer_addr
    ))).serialize();
    broadcast(&clients, &msg);

    loop {
        match crate::framing::read_message(&mut stream) {
            Ok(data) => {
                println!("Received {} bytes from {}", data.len(), peer_addr);
                broadcast(&clients, &data)
            },
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(err) => return Err(err)
        }
    }

    clients.lock().unwrap().retain(|c| c.peer_addr().ok() != Some(peer_addr));

    println!("Disconnected: {}", stream.peer_addr()?);

    let msg = Message::System(SystemMessage::new(format!(
        "{} ({}) disconnected",
        std::str::from_utf8(&client_name)
            .unwrap_or("\u{FFFD}")
            .trim_end_matches('\0')
            .trim_end(),
        peer_addr
    ))).serialize();
    broadcast(&clients, &msg);

    Ok(())
}

fn broadcast(clients: &ClientList, buffer: &[u8]) {
    let mut guard = clients.lock().unwrap();
    for client in guard.iter_mut() {
        let _ = crate::framing::write_message(client, &buffer);
    }
}
