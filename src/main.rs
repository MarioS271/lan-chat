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

use crate::args::{ClientCommand, Command, ServerCommand};

pub const DEFAULT_PORT: u16 = 42003;
pub const PROTOCOL_VERSION: u16 = 1;

fn main() {
    let command = args::parse(std::env::args().skip(1));

    match command {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(Command::Help) => help_root(),
        Ok(Command::Version) => println!("Protocol Version: {}", PROTOCOL_VERSION),

        Ok(Command::Client(cmd)) => match cmd {
            ClientCommand::Help => help_client(),
            ClientCommand::List => todo!("client list"),
            ClientCommand::Add { name } => todo!("client add"),
            ClientCommand::Connect { name } => todo!("client connect"),
        }

        Ok(Command::Server(cmd)) => match cmd {
            ServerCommand::Help => help_server(),
            ServerCommand::List => todo!("server list"),
            ServerCommand::ShowKey { name } => todo!("server showkey"),
            ServerCommand::New { name } => todo!("server new"),
            ServerCommand::Start { name } => todo!("server start")
        }
    };
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
