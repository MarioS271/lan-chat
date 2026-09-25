// SPDX-License-Identifier: GPL-3.0-only
//! Client State Struct
//!
//! Authors: MarioS271

use crate::message::Message;

pub struct ClientState {
    pub name: String,
    pub remote: String,
    pub messages: Vec<Message>,
    pub input: String
}
