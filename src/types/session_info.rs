// SPDX-License-Identifier: GPL-3.0-only
//! Session Info Struct
//!
//! Authors: MarioS271

use std::net::{Ipv4Addr, TcpStream};
use crate::helpers::ask_for_input;

pub struct SessionInfo {
    pub name: [u8; Self::MAX_NAME_LEN],
    pub ip: Ipv4Addr
}

impl SessionInfo {
    pub const MAX_NAME_LEN: usize = 32;

    pub fn read_in_name(&mut self) -> std::io::Result<()> {
        let name = ask_for_input("Enter your name")?;
        let bytes = name.trim().as_bytes();

        if bytes.len() > Self::MAX_NAME_LEN {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Name too long"
            ));
        }

        self.name[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn read_in_ip(&mut self, stream: &TcpStream) -> std::io::Result<()> {
        match stream.local_addr()? {
            std::net::SocketAddr::V4(addr) => {
                self.ip = *addr.ip();
                Ok(())
            },
            std::net::SocketAddr::V6(_) => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "IPv6 is not supported"
                ))
            }
        }
    }

    pub fn name_as_str(&self) -> &str {
        std::str::from_utf8(&self.name)
            .unwrap_or("?")
            .trim_end_matches('\0')
    }
}
impl Default for SessionInfo {
    fn default() -> Self {
        Self {
            name: [0u8; Self::MAX_NAME_LEN],
            ip: Ipv4Addr::UNSPECIFIED
        }
    }
}
