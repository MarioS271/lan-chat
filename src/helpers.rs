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

pub fn ask_for_input(prompt: &str) -> std::io::Result<String> {
    print!("{}\n > ", prompt);

    let stdin = std::io::stdin();
    let mut result = String::new();

    if stdin.read_line(&mut result).is_err() || result.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid Input"
        ));
    }

    Ok(result)
}
