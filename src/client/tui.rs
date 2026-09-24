// SPDX-License-Identifier: GPL-3.0-only
//! Client TUI Logic
//!
//! Authors: MarioS271

use crate::client::receive::receive_thread;
use crate::client::state::AppState;
use crate::framing;
use crate::types::message::Message;
use crate::types::session_info::SessionInfo;
use ratatui::crossterm;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::Constraint;
use ratatui::widgets::Paragraph;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

const LOG_PREFIX: &str = "(main thread)";

pub fn init_tui(session_info: SessionInfo, mut stream: TcpStream) -> std::io::Result<()> {
    let _guard = TerminalGuard;

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen
    )?;

    let mut terminal = ratatui::Terminal::new(
        ratatui::backend::CrosstermBackend::new(std::io::stdout())
    )?;

    let state = Arc::new(Mutex::new(AppState {
        name: session_info.name_as_str().to_string(),
        remote: stream.peer_addr()?.to_string(),
        messages: Vec::new(),
        input: String::new()
    }));

    let read_stream = stream.try_clone()?;
    let state_recv = Arc::clone(&state);

    std::thread::spawn(move || receive_thread(read_stream, state_recv));

    loop {
        terminal.draw(|frame| {
            render_tui(frame, &state.lock().unwrap());
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(50))? {
            match crossterm::event::read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char(c) => {
                            state.lock().unwrap().input.push(c);
                        }
                        KeyCode::Backspace => {
                            state.lock().unwrap().input.pop();
                        }
                        KeyCode::Enter => {
                            let input = {
                                let mut state = state.lock().unwrap();
                                let msg = state.input.trim().to_string();
                                state.input.clear();
                                msg
                            };
                            if !input.is_empty() {
                                let msg = Message::new(&session_info, &input);
                                if let Err(err) = framing::write_message(
                                    &mut stream,
                                    msg.serialize().as_slice()
                                ) {
                                    end_raw_mode();
                                    eprintln!("{} Write Error: {}", LOG_PREFIX, err);
                                }
                            }
                        }
                        KeyCode::Esc => {
                            end_raw_mode();
                            std::process::exit(0);
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn render_tui(frame: &mut ratatui::Frame, state: &AppState) {
    let areas = ratatui::layout::Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(3)
    ]).split(frame.area());

    let msg_lines: Vec<ratatui::text::Line> = state.messages.iter().map(|msg| {
        let timestamp = chrono::DateTime::from_timestamp(msg.timestamp_secs as i64, 0)
            .unwrap()
            .with_timezone(&chrono::Local)
            .format("%H:%M:%S");

        let name = std::str::from_utf8(&msg.sender_name)
            .unwrap_or("???")
            .trim_end_matches('\0')
            .trim_end();

        let content = String::from_utf8_lossy(&msg.content);

        ratatui::text::Line::from(format!("[{}] {}: {}", timestamp, name, content.trim_end()))
    }).collect();

    frame.render_widget(
        Paragraph::new(format!("lan-chat | Connected to {} as {}", state.remote, state.name))
            .style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED)),
        areas[0]
    );

    frame.render_widget(
        Paragraph::new(msg_lines)
            .block(ratatui::widgets::Block::bordered().title("Messages")),
        areas[1]
    );

    let available_width = (areas[2].width - 2) as usize;
    let display_input = if state.input.len() > available_width {
        &state.input[state.input.len() - available_width..]
    } else {
        &state.input
    };

    frame.render_widget(
        Paragraph::new(format!(" {}", display_input))
            .block(ratatui::widgets::Block::bordered().title("Input")),
        areas[2]
    );

    let cursor_x = areas[2].x + 2 + display_input.len() as u16;
    let cursor_y = areas[2].y + 1;
    frame.set_cursor_position(ratatui::layout::Position { x: cursor_x, y: cursor_y });
}

struct TerminalGuard;
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        end_raw_mode()
    }
}
pub fn end_raw_mode() {
    let _ = crossterm::terminal::disable_raw_mode();
    let _ = crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen
    );
    let _ = crossterm::execute!(
        std::io::stdout(),
        crossterm::cursor::Show
    );
}
