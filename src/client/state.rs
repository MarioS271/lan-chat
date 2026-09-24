// SPDX-License-Identifier: GPL-3.0-only
//! Client App State Struct
//!
//! Authors: MarioS271

use crate::types::message::Message;

pub struct AppState {
    pub name: String,
    pub remote: String,
    pub messages: Vec<Message>,
    pub input: String
}
