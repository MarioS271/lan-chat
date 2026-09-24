// SPDX-License-Identifier: GPL-3.0-only
//! Server Mode Logic
//!
//! Authors: MarioS271

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
        std::thread::spawn(move || {
            handle_client(stream, clients);
        });
    }

    Ok(())
}

fn handle_client(stream: TcpStream, clients: ClientList) {
    let write_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\nFailed to clone stream: {}", e);
            return;
        }
    };

    clients.lock().unwrap().push(write_stream);

    let mut reader = stream;
    loop {
        match crate::framing::read_message(&mut reader) {
            Ok(data) => {
                println!("Recieved {} bytes", data.len());

                let mut guard = clients.lock().unwrap();

                for client in guard.iter_mut() {
                    if let Err(err) = crate::framing::write_message(client, &data) {
                        eprintln!("Broadcast Write Error: {}", err);
                    }
                }
            },
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("Client disconnected");
                break;
            },
            Err(err) => {
                eprintln!("Read Error: {}", err);
                break;
            }
        }
    }
}
