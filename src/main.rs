use std::time::Duration;

use clap::Parser;
use cli::Cli;
use player::Player;
use smol::Timer;

use glib::FlagsClass;
use gstreamer::prelude::*;

mod cli;
mod player;

fn main() -> Result<(), anyhow::Error> {
    // Set up main loop
    // let main_loop = glib::MainLoop::new(None, false);

    // Initialize GStreamer
    gstreamer::init()?;

    // let uri = "https://gstreamer.freedesktop.org/data/media/sintel_cropped_multilingual.webm";
    let uri = "https://gstreamer.freedesktop.org/data/media/sintel_cropped_multilingual.webm";

    // Create PlayBin element
    let playbin = gstreamer::ElementFactory::make("playbin")
        .name("playbin")
        // Set URI to play
        .property("uri", uri)
        // Set connection speed. This will affect some internal decisions of playbin
        // .property("connection-speed", 56u64)
        .build()?;

    // Set flags to show Audio and Video but ignore Subtitles
    let flags = playbin.property_value("flags");
    let flags_class = FlagsClass::with_type(flags.type_()).unwrap();

    let flags = flags_class
        .builder_with_value(flags)
        .unwrap()
        .set_by_nick("audio")
        .unset_by_nick("video")
        .unset_by_nick("text")
        .build()
        .unwrap();
    playbin.set_property_from_value("flags", &flags);

    // Handle keyboard input
    // thread::spawn(move || handle_keyboard(&playbin_clone, &main_loop_clone));

    // Add a bus watch, so we get notified when a message arrives
    let playbin_clone = playbin.clone();
    // let main_loop_clone = main_loop.clone();
    let bus = playbin.bus().unwrap();
    let _bus_watch = bus.add_watch(move |_bus, message| {
        println!("Got message on bus: {:?}", message);

        use gstreamer::MessageView;
        match message.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?} {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                // main_loop_clone.quit();
                glib::ControlFlow::Break
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed
                    .src()
                    .map(|s| s == &playbin_clone)
                    .unwrap_or(false)
                    && state_changed.current() == gstreamer::State::Playing
                {
                    // analyze_streams(&playbin_clone);
                }
                glib::ControlFlow::Continue
            }
            MessageView::Eos(..) => {
                println!("Reached end of stream");
                // main_loop_clone.quit();
                glib::ControlFlow::Break
            }
            _ => glib::ControlFlow::Continue,
        }
    })?;

    // Set to PLAYING
    playbin.set_state(gstreamer::State::Playing)?;

    // Set GLib mainlooop to run
    // main_loop.run();

    // Clean up
    // playbin.set_state(gstreamer::State::Null)?;

    // Ok(())

    // let cli = Cli::parse();
    // let Some(url) = cli.url() else {
    //     anyhow::bail!("No such file or invalid URL: {}", cli.media);
    // };

    // println!("Playing {}", url.as_str());

    // // let mut player = Player::new(url)?;

    // // player.play()?;

    // let main_loop = glib::MainLoop::new(None, false);

    // let uri = "https://gstreamer.freedesktop.org/data/media/sintel_cropped_multilingual.webm";

    // gstreamer::init()?;

    // // Create PlayBin element
    // let playbin = gstreamer::ElementFactory::make("playbin")
    //     .name("playbin")
    //     .property("uri", uri)
    //     .build()?;

    // let flags = playbin.property_value("flags");
    // let flags_class = FlagsClass::with_type(flags.type_()).unwrap();

    // let flags = flags_class
    //     .builder_with_value(flags)
    //     .unwrap()
    //     .set_by_nick("audio")
    //     .set_by_nick("video")
    //     .unset_by_nick("text")
    //     .build()
    //     .unwrap();
    // playbin.set_property_from_value("flags", &flags);

    // let playbin_clone = playbin.clone();
    // let main_loop_clone = main_loop.clone();
    // let bus = playbin.bus().unwrap();
    // let _bus_watch = bus.add_watch(move |_bus, message| {
    //     use gstreamer::MessageView;

    //     println!("Found message: {:?}", message);

    //     match message.view() {
    //         MessageView::Error(err) => {
    //             eprintln!(
    //                 "Error received from element {:?} {}",
    //                 err.src().map(|s| s.path_string()),
    //                 err.error()
    //             );
    //             eprintln!("Debugging information: {:?}", err.debug());
    //             main_loop_clone.quit();
    //             glib::ControlFlow::Break
    //         }
    //         MessageView::StateChanged(state_changed) => {
    //             if state_changed
    //                 .src()
    //                 .map(|s| s == &playbin_clone)
    //                 .unwrap_or(false)
    //                 && state_changed.current() == gstreamer::State::Playing
    //             {
    //                 println!("Now playing");
    //                 // analyze_streams(&playbin_clone);
    //             }
    //             glib::ControlFlow::Continue
    //         }
    //         MessageView::Eos(..) => {
    //             println!("Reached end of stream");
    //             main_loop_clone.quit();
    //             glib::ControlFlow::Break
    //         }
    //         _ => glib::ControlFlow::Continue,
    //     }
    // })?;

    // playbin.set_state(gstreamer::State::Playing)?;

    // main_loop.run();
    // Ok(())

    smol::block_on(main_loop())
}

async fn main_loop() -> Result<(), anyhow::Error> {
    loop {
        // let time = player.current_position();
        // let playing = player.is_playing();

        // println!("Player at {:?}, currently playing: {}", time, playing);

        // if !playing {
        // player.play()?;
        // }

        println!("Loop");

        Timer::after(Duration::from_secs(1)).await;
    }
}
