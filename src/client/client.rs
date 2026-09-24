// SPDX-License-Identifier: GPL-3.0-only
//! Client Mode Logic
//!
//! Authors: MarioS271

use crate::client::tui::init_tui;
use crate::types::session_info::SessionInfo;
use std::net::TcpStream;

pub fn run_client(address: &str) -> std::io::Result<()> {
    println!("Running as client");

    let mut session_info = SessionInfo::default();
    session_info.read_in_name()?;

    println!("Attempting to connect to {}", address);

    let stream = TcpStream::connect(address)?;
    stream.set_nodelay(true)?;

    println!("Successfully connected to {}", stream.peer_addr()?);

    session_info.read_in_ip(&stream)?;

    init_tui(session_info, stream)?;
    Ok(())
}
