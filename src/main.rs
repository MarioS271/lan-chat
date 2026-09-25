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

use qrcode::QrCode;
use qrcode::render::unicode;
use crate::args::{ClientCommand, Command, ServerCommand};

pub const PROTOCOL_VERSION: u16 = 1;
pub const DEFAULT_PORT: u16 = 42003;

fn main() {
    let command = args::parse(std::env::args().skip(1));

    let run = || -> Result<(), String> {
        match command? {
            Command::Help => Ok(help_root()),
            Command::Version => Ok(println!("Protocol Version: {}", PROTOCOL_VERSION)),

            Command::Client(cmd) => match cmd {
                ClientCommand::Help => Ok(help_client()),
                ClientCommand::List => {
                    let config = config::client::load()?;

                    if !config.servers.is_empty() {
                        println!("Known Servers:");
                    } else {
                        println!("No servers currently known");
                        return Ok(());
                    }

                    for server in config.servers {
                        println!("  {} at {}", server.name, server.address);
                    }

                    Ok(())
                },
                ClientCommand::Add { name } => todo!("client add"),
                ClientCommand::Connect { name } => todo!("client connect"),
            }

            Command::Server(cmd) => match cmd {
                ServerCommand::Help => Ok(help_server()),
                ServerCommand::List => {
                    let config = config::server::load()?;

                    if !config.servers.is_empty() {
                        println!("Existing Servers:");
                    } else {
                        println!("No servers exist");
                        return Ok(());
                    }

                    for server in config.servers {
                        println!("  {} on port {}", server.name, server.port);
                    }

                    Ok(())
                },
                ServerCommand::ShowKey { name } => {
                    let config = config::server::load()?;
                    let server = config.servers.iter()
                        .find(|server| server.name == name)
                        .ok_or(format!("Server '{}' not found", name))?;

                    let qr_code = QrCode::new(server.key.as_bytes())
                        .map_err(|e| format!("Could not generate QR code: {}", e))?;
                    let qr_image = qr_code.render::<unicode::Dense1x2>().build();

                    println!("Key of {}: {}", server.name, server.key);
                    println!("{}", qr_image);

                    Ok(())
                },
                ServerCommand::New { name } => todo!("server new"),
                ServerCommand::Start { name } => todo!("server start")
            }
        }
    };

    if let Err(e) = run() {
        eprintln!("{}", e);
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
            --new       Add a server with the given name
            --show-key  Print the given server's encryption key as text
                        and as a QR code
            --help      Show this message
    "#});
}
