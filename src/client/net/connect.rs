// SPDX-License-Identifier: GPL-3.0-only
//! Client Connect and Handshake
//!
//! Authors: MarioS271

use crate::client::session_info::SessionInfo;
use crate::client::ui::tui::init_tui;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn connect(address: &str) -> std::io::Result<()> {
    println!("Running as client");

    let mut session_info = SessionInfo::default();
    session_info.read_in_name()?;

    println!("Attempting to connect to {}", address);

    let mut stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;

    println!("Successfully connected to {}", stream.peer_addr()?);

    let mut server_version = [0u8; 2];
    stream.read_exact(&mut server_version)?;

    if u16::from_be_bytes(server_version) != crate::PROTOCOL_VERSION {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "This client is incompatible with the server's protocol version"
        ));
    }

    stream.write_all(&crate::PROTOCOL_VERSION.to_be_bytes())?;
    stream.write_all(&session_info.name)?;

    init_tui(session_info, stream)?;
    Ok(())
}
