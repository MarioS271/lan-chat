// SPDX-License-Identifier: GPL-3.0-only
//! Session Info Struct
//!
//! Authors: MarioS271

use std::net::Ipv4Addr;

pub struct SessionInfo {
    pub name: [u8; 32],
    pub ip: Ipv4Addr
}
