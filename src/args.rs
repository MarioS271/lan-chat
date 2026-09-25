// SPDX-License-Identifier: GPL-3.0-only
//! Argument Parsing
//!
//! Authors: MarioS271

pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Command, String> {
    match args.next().as_deref() {
        None | Some("--help") => Ok(Command::Help),
        Some("--version") => Ok(Command::Version),

        Some("client") => Ok(Command::Client(parse_client(args)?)),
        Some("server") => Ok(Command::Server(parse_server(args)?)),

        Some(other) => Err(format!("Unknown Option or Subcommand: {}", other))
    }
}

fn parse_client(args: impl Iterator<Item = String>) -> Result<ClientCommand, String> {
    let mut name: String = String::new();
    let mut flag_add = false;

    for arg in args {
        match arg.as_str() {
            "--help" => return Ok(ClientCommand::Help),
            "--list" => return Ok(ClientCommand::List),
            "--add" => flag_add = true,
            other if other.starts_with("--") => return Err(format!("Unknown Option: {}", other)),
            other => name = other.to_string()
        }
    }

    if name.is_empty() && !flag_add {
        return Ok(ClientCommand::Help);
    }
    if name.is_empty() {
        return Err("Missing 'name' argument".to_string());
    }

    if flag_add {
        return Ok(ClientCommand::Add { name });
    }

    Ok(ClientCommand::Connect { name })
}

fn parse_server(args: impl Iterator<Item = String>) -> Result<ServerCommand, String> {
    let mut name: String = String::new();
    let mut flag_new = false;
    let mut flag_show_key = false;

    for arg in args {
        match arg.as_str() {
            "--help" => return Ok(ServerCommand::Help),
            "--list" => return Ok(ServerCommand::List),
            "--new" => flag_new = true,
            "--show-key" => flag_show_key = true,
            other if other.starts_with("--") => return Err(format!("Unknown Option: {}", other)),
            other => name = other.to_string()
        }
    }

    if name.is_empty() && !flag_new && !flag_show_key {
        return Ok(ServerCommand::Help);
    }
    if name.is_empty() {
        return Err("Missing 'name' argument".to_string());
    }

    if flag_new && flag_show_key {
        return Err("--new and --show-key can't be used together".to_string());
    }
    if flag_new {
        return Ok(ServerCommand::New { name })
    }
    if flag_show_key {
        return Ok(ServerCommand::ShowKey { name })
    }

    Ok(ServerCommand::Start { name })
}

pub enum Command {
    Client(ClientCommand),
    Server(ServerCommand),
    Version,
    Help
}

pub enum ClientCommand {
    Connect { name: String },
    Add { name: String },
    List,
    Help
}

pub enum ServerCommand {
    Start { name: String },
    New { name: String },
    ShowKey { name: String },
    List,
    Help
}
