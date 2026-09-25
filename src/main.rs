// SPDX-License-Identifier: GPL-3.0-only
//! Program Entrypoint
//!
//! Authors: MarioS271

mod client;
mod config;
mod encryption;
mod server;

mod framing;
mod helpers;
mod message;
mod args;

pub const DEFAULT_PORT: u16 = 42003;
pub const PROTOCOL_VERSION: u16 = 1;

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
        "-h" | "--help" => {
            help_root();
            std::process::exit(0);
        }
        "-v" | "--version" => {
            println!("Protocol Version: {}", PROTOCOL_VERSION);
            std::process::exit(0);
        },
        "-c" | "--client" => 0,
        "-s" | "--server" => 1,
        _ => {
            if mode_arg.is_empty() {
                help_root();
                std::process::exit(0);
            }

            eprintln!("Error: invalid or unknown argument: {}", mode_arg);

            std::process::exit(1);
        }
    };

    if let Err(e) = match mode {
        0 => {
            let address = match ip_and_or_port_arg.contains(":") {
                true => ip_and_or_port_arg.as_str(),
                false => &format!("{}:{}", ip_and_or_port_arg, DEFAULT_PORT)
            };
            client::net::connect::connect(address)
        }
        _ => {
            let port = ip_and_or_port_arg.parse::<u16>().unwrap_or(DEFAULT_PORT);
            server::server::run_server(port)
        }
    } {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn help_root() {
    println!(indoc::indoc! {r#"
        Usage: simple-chat <subcommand> [options]

        Subcommands:
            client      Connect to or register a server
            server      Run or manage a server instance

        Options:
            --help      Show this message
            --version   Show this binary's protocol version

        Run 'simple-chat <subcommand> --help' for
        subcommand-specific help.
    "#});
}
fn help_client() {
    println!(indoc::indoc! {r#"
        Usage: simple-chat client <name> [options]

        Arguments:
            name        Name of the server to connect to/to add
                        (prompted if omitted)

        Options:
            --list      Output a list of available servers
            --add       Add a server with the given name, will prompt
                        for the encryption key (and name if omitted)
            --help      Show this message
    "#});
}
fn help_server() {
    println!(indoc::indoc! {r#"
        Usage: simple-chat server <name> [options]

        Arguments:
            name        Name of the server to/to add
                        (prompted if omitted)

        Options:
            --list      Output a list of available servers
            --new       Add a server with the given name, will prompt
            --show-key  Print the given server's encryption key as text
                        and as a QR code
            --help      Show this message
    "#});
}
