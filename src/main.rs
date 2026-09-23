// SPDX-License-Identifier: GPL-3.0-only
//! Main Function
//!
//! Authors: MarioS271

mod modes;
mod server;

use crate::modes::Modes;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();

    let mode_arg = args
        .next()
        .unwrap_or_else(|| "-h".to_string());

    let port_arg = args
        .next()
        .unwrap_or_default();

    let mode = match mode_arg.as_str() {
        "-c" | "--client" => Modes::Client,
        "-s" | "--server" => Modes::Server,
        "-h" | "--help" => print_help_and_exit(),
        _ => {
            if mode_arg.is_empty() {
                print_help_and_exit();
            }

            eprintln!("Error: invalid or unknown argument: {}", mode_arg);

            std::process::exit(1);
        }
    };

    if port_arg.is_empty() {
        eprintln!("Error: no port given");
        std::process::exit(1);
    }

    let port = port_arg.parse::<u16>().unwrap_or_else(
        |_| {
            eprintln!("Error: could not parse given port number");
            std::process::exit(1);
        }
    );
}

fn print_help_and_exit() -> ! {
    println!("lan-chat – A LAN TCP messaging TUI app\n");
    println!("Usage: lan-chat <flag> <port>");
    println!("  -c --client: Run this instance as a client which can connect to a server instance in the same LAN");
    println!("  -s --server: Run this instance as a server anyone in the same LAN can connect to");
    println!("  -h --help: Print this message");

    std::process::exit(0);
}
