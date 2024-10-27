use std::thread;

use smol::stream::{Stream, StreamExt};
use termion::{event::Key, input::TermRead};

#[derive(Debug)]
pub enum Command {
    TogglePlaying,
    Quit,
}

pub fn commands() -> impl Stream<Item = Command> {
    keys().map(|key| Command::Quit)
}

fn keys() -> impl Stream<Item = Key> {
    let (send, recv) = smol::channel::unbounded();

    let tty = termion::get_tty().expect("Reading from TTY should succeed");

    thread::spawn(move || {
        for key in tty.keys() {
            let key = key.expect("Reading key from TTY should succeed");
            println!("Found key: {:?}", key);
            let res = send.send_blocking(key);

            if res.is_err() {
                // Channel is closed
                return;
            }
        }
    });

    recv
}
