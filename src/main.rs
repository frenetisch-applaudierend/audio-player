use std::{io::stdout, time::Duration};

use clap::Parser;
use cli::Cli;
use commands::commands;
use player::Player;
use smol::{
    stream::{Stream, StreamExt},
    Timer,
};
use termion::raw::IntoRawMode;

mod cli;
mod commands;
mod player;
mod ui;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let Some(url) = cli.url() else {
        anyhow::bail!("No such file or invalid URL: {}", cli.media);
    };

    println!("Playing {}", url.as_str());

    let mut out = std::io::stdout().into_raw_mode()?;

    let mut player = Player::new(url)?;

    player.play()?;

    // // Add a bus watch, so we get notified when a message arrives
    // let bus = playbin.bus().unwrap();
    // let _bus_watch = bus.add_watch(move |_bus, message| {
    //     println!("Got message on bus: {:?}", message);
    //     glib::ControlFlow::Continue
    // })?;

    smol::block_on(main_loop(player))
}

async fn main_loop(mut player: Player) -> Result<(), anyhow::Error> {
    let commands = commands().map(|c| Event::Command(c));
    let ticks = ticks().map(|_| Event::Tick);

    let mut events = commands.or(ticks).boxed();

    while let Some(event) = events.next().await {
        if let Event::Command(cmd) = event {
            println!("{:?}", cmd);
        }

        ui::draw_frame(&mut player);
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
