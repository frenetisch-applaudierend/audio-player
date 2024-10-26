use std::time::Duration;

use clap::Parser;
use cli::Cli;
use player::Player;
use smol::Timer;

mod cli;
mod player;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let Some(url) = cli.url() else {
        anyhow::bail!("No such file or invalid URL: {}", cli.media);
    };

    println!("Playing {}", url.as_str());

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
    loop {
        let time = player.current_position();
        let duration = player.duration();
        let playing = player.is_playing();

        println!(
            "Player at {:?} / {:?}, currently playing: {}",
            time, duration, playing
        );

        Timer::after(Duration::from_secs(1)).await;
    }
}
