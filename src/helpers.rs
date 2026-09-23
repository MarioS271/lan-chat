// SPDX-License-Identifier: GPL-3.0-only
//! Helpers such as [`get_timestamp`]
//!
//! Authors: MarioS271

pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}