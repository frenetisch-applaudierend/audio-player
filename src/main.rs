use std::{collections::HashMap, time::Duration};

use clap::Parser;
use smol::stream::{Stream, StreamExt};

use cli::Cli;
use commands::commands;
use player::Player;
use ui::UserInterface;

mod cli;
mod commands;
mod player;
mod ui;

struct State {
    player: Player,
    ui: UserInterface,
    saved_positions: HashMap<char, Duration>,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let Some(url) = cli.url() else {
        anyhow::bail!("No such file or invalid URL: {}", cli.media);
    };

    println!("Playing {}", url.as_str());

    let player = Player::new(url)?;
    let ui = UserInterface::setup()?;
    let mut state = State {
        player,
        ui,
        saved_positions: HashMap::new(),
    };

    state.player.play()?;

    // // Add a bus watch, so we get notified when a message arrives
    // let bus = playbin.bus().unwrap();
    // let _bus_watch = bus.add_watch(move |_bus, message| {
    //     println!("Got message on bus: {:?}", message);
    //     glib::ControlFlow::Continue
    // })?;

    let res = smol::block_on(main_loop(&mut state));

    let State {
        player: _,
        ui,
        saved_positions: _,
    } = state;

    ui.restore()?;

    res
}

async fn main_loop(state: &mut State) -> Result<(), anyhow::Error> {
    let commands = commands().map(|c| Event::Command(c));
    let ticks = ticks().map(|_| Event::Tick);

    let mut events = commands.or(ticks).boxed();

    while let Some(event) = events.next().await {
        if let Event::Command(cmd) = event {
            use commands::Command::*;

            match cmd {
                TogglePlaying => state.player.toggle_playing()?,
                SavePosition(pos) => {
                    state
                        .saved_positions
                        .insert(pos, state.player.current_position());
                }
                RestorePosition(pos) => {
                    if let Some(duration) = state.saved_positions.get(&pos) {
                        state.player.seek_to(*duration)?;
                    }
                }
                Quit => return Ok(()),
            }
        }

        state.ui.draw_frame(&mut state.player)?;
    }

    Ok(())
}

fn ticks() -> impl Stream<Item = ()> {
    smol::Timer::interval(Duration::from_millis(100)).map(|_| ())
}

#[derive(Debug)]
enum Event {
    Command(commands::Command),
    Tick,
}
