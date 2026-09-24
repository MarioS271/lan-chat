// SPDX-License-Identifier: GPL-3.0-only
//! Message Definition
//!
//! Authors: MarioS271

use crate::helpers;
use crate::types::session_info::SessionInfo;
use std::net::Ipv4Addr;

pub struct Message {
    pub timestamp_secs: u64,
    pub sender_name: [u8; SessionInfo::MAX_NAME_LEN],
    pub sender_ip: Ipv4Addr,
    pub content: Vec<u8>
}

impl Message {
    pub const MESSAGE_VERSION: u16 = 1;

    const METADATA_BYTES: usize = 46;

    const VERSION_BYTES: usize = 2;
    const TIMESTAMP_BYTES: usize = 8;
    const SENDER_NAME_BYTES: usize = SessionInfo::MAX_NAME_LEN;
    const SENDER_IP_BYTES: usize = 4;

    const VERSION_OFFSET: usize = 0;
    const TIMESTAMP_OFFSET: usize = Self::VERSION_OFFSET + Self::VERSION_BYTES;
    const SENDER_NAME_OFFSET: usize = Self::TIMESTAMP_OFFSET + Self::TIMESTAMP_BYTES;
    const SENDER_IP_OFFSET: usize = Self::SENDER_NAME_OFFSET + Self::SENDER_NAME_BYTES;
    const CONTENT_OFFSET: usize = Self::SENDER_IP_OFFSET + Self::SENDER_IP_BYTES;

    pub fn new(session_info: &SessionInfo, content: &str) -> Self {
        let mut message = Self {
            timestamp_secs: helpers::get_timestamp(),
            sender_name: session_info.name,
            sender_ip: session_info.ip,
            content: Vec::new(),
        };

        message.content.extend_from_slice(content.as_bytes());

        message
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::METADATA_BYTES + self.content.len());

        buf.extend_from_slice(&Self::MESSAGE_VERSION.to_be_bytes());
        buf.extend_from_slice(&self.timestamp_secs.to_be_bytes());
        buf.extend_from_slice(&self.sender_name);
        buf.extend_from_slice(&self.sender_ip.to_bits().to_be_bytes());
        buf.extend_from_slice(&self.content);

        buf
    }

    pub fn deserialize(buffer: &[u8]) -> std::io::Result<Self> {
        if buffer.len() <= Self::METADATA_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer too short"
            ));
        }

        let version = u16::from_be_bytes(
            buffer[Self::VERSION_OFFSET..Self::TIMESTAMP_OFFSET]
                .try_into()
                .unwrap()
        );
        if version != Self::MESSAGE_VERSION {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Version mismatch"
            ));
        }

        let timestamp_secs = u64::from_be_bytes(
            buffer[Self::TIMESTAMP_OFFSET..Self::SENDER_NAME_OFFSET]
                .try_into()
                .unwrap()
        );
        let sender_name = buffer[Self::SENDER_NAME_OFFSET..Self::SENDER_IP_OFFSET].try_into().unwrap();
        let sender_ip = Ipv4Addr::from(u32::from_be_bytes(
            buffer[Self::SENDER_IP_OFFSET..Self::CONTENT_OFFSET]
                .try_into()
                .unwrap()
        ));
        let content = buffer[Self::CONTENT_OFFSET..].to_vec();

        Ok(Self {
            timestamp_secs,
            sender_name,
            sender_ip,
            content
        })
    }
}
