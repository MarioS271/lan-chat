// SPDX-License-Identifier: GPL-3.0-only
//! Main Function
//!
//! Authors: MarioS271

mod client;
mod types;
mod framing;
mod helpers;
mod server;

use client::client::run_client;
use server::run_server;
use types::message::Message;
use types::modes::Modes;

pub const DEFAULT_PORT: u16 = 42003;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();

    let mode_arg = args
        .next()
        .unwrap_or_else(|| "-h".to_string());

    let ip_and_or_port_arg = args
        .next()
        .unwrap_or_default();

    let mode = match mode_arg.as_str() {
        "-h" | "--help" => print_help_and_exit(),
        "-v" | "--version" => {
            println!("Message Version: {}", Message::MESSAGE_VERSION);
            std::process::exit(0);
        },
        "-c" | "--client" => Modes::Client,
        "-s" | "--server" => Modes::Server,
        _ => {
            if mode_arg.is_empty() {
                print_help_and_exit();
            }

            eprintln!("Error: invalid or unknown argument: {}", mode_arg);

            std::process::exit(1);
        }
    };

    if let Err(e) = match mode {
        Modes::Client => {
            let address = match ip_and_or_port_arg.contains(":") {
                true => ip_and_or_port_arg.as_str(),
                false => &format!("{}:{}", ip_and_or_port_arg, DEFAULT_PORT)
            };
            run_client(address)
        }
        Modes::Server => {
            let port = ip_and_or_port_arg.parse::<u16>().unwrap_or(DEFAULT_PORT);
            run_server(port)
        }
    } {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn print_help_and_exit() -> ! {
    println!("lan-chat – A simple CLI LAN messenger");
    println!("Message Version: {}\n", Message::MESSAGE_VERSION);

    println!("Usage: lan-chat <flag> <port|ip:port>");
    println!("  -h --help: Print this message");
    println!("  -v --version: Output this binary's message version");
    println!("  -c --client: Run this instance as a client which can connect to a server instance in the same LAN");
    println!("               You also need to supply a <ip:port> pair to be able to connect to a server.");
    println!("  -s --server: Run this instance as a server anyone in the same LAN can connect to");
    println!("               You also need to supply a <port> on which clients will connect.");

    std::process::exit(0);
}
