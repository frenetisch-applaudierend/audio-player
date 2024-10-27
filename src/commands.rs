use std::thread;

use smol::stream::{Stream, StreamExt};
use termion::{event::Key, input::TermRead};

#[derive(Debug)]
pub enum Command {
    TogglePlaying,
    SavePosition(char),
    RestorePosition(char),
    Quit,
}

pub fn commands() -> impl Stream<Item = Command> {
    keys().filter_map(|key| match key {
        Key::Char(' ') => Some(Command::TogglePlaying),
        Key::Alt(pos) if pos.is_ascii_digit() => Some(Command::SavePosition(pos)),
        Key::Char(pos) if pos.is_ascii_digit() => Some(Command::RestorePosition(pos)),
        Key::Char('q') => Some(Command::Quit),
        Key::Ctrl('c') => Some(Command::Quit),
        Key::Esc => Some(Command::Quit),
        _ => None,
    })
}

fn keys() -> impl Stream<Item = Key> {
    let (send, recv) = smol::channel::unbounded();

    let tty = termion::get_tty().expect("Reading from TTY should succeed");

    thread::spawn(move || {
        for key in tty.keys() {
            let key = key.expect("Reading key from TTY should succeed");
            let res = send.send_blocking(key);

            if res.is_err() {
                // Channel is closed
                return;
            }
        }
    });

    recv
}
