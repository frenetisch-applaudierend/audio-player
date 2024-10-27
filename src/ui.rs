use std::io::Write;

use anyhow::Context;
use termion::{
    cursor::{DetectCursorPos, HideCursor},
    raw::{IntoRawMode, RawTerminal},
};

use crate::player::Player;

pub struct UserInterface {
    out: HideCursor<RawTerminal<std::io::Stdout>>,
    initial_cursor_pos: (u16, u16),
}

impl UserInterface {
    pub fn setup() -> Result<Self, anyhow::Error> {
        let mut out = std::io::stdout()
            .into_raw_mode()
            .context("Entering raw mode failed")?;

        write!(out, "{}", termion::cursor::Save).context("Setting up terminal failed")?;

        let mut out = HideCursor::from(out);

        let initial_cursor_pos = out
            .cursor_pos()
            .context("Could not get current curser position")?;

        Ok(Self {
            out,
            initial_cursor_pos,
        })
    }

    pub fn restore(mut self) -> Result<(), anyhow::Error> {
        write!(self.out, "{}", termion::cursor::Restore)
            .context("Failed to reset the cursor. Type 'reset' to restore it manually.")?;

        self.out
            .suspend_raw_mode()
            .context("Failed to exit raw mode. Type 'reset' to restore your terminal manually.")?;

        Ok(())
    }

    pub fn draw_frame(&mut self, player: &mut Player) -> Result<(), anyhow::Error> {
        let time = player.current_position();
        let duration = player.duration();
        let playing = player.is_playing();

        write!(
            self.out,
            "{}{}Player at {} / {}, currently playing: {}",
            termion::cursor::Goto(self.initial_cursor_pos.0, self.initial_cursor_pos.1),
            termion::clear::CurrentLine,
            PlayTime(time, true),
            PlayTime(duration, false),
            playing
        )
        .context("Could not write to the terminal")?;

        self.out.flush().context("Could not flush output")?;

        Ok(())
    }
}

struct PlayTime(std::time::Duration, bool);

impl std::fmt::Display for PlayTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_seconds = self.0.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            write!(f, "{}:", hours)?;
        }
        write!(f, "{}:{:0>2}", minutes, seconds)?;

        if self.1 {
            let subsecs = self.0.subsec_millis() / 100;
            write!(f, ".{}", subsecs)?;
        }

        Ok(())
    }
}
