use std::time::Duration;

use anyhow::Context;
use gstreamer::{
    prelude::{ElementExt, ElementExtManual, ObjectExt},
    ClockTime,
};
use url::Url;

pub struct Player {
    playbin: gstreamer::Element,
    duration: Option<Duration>,
}

impl Player {
    pub(crate) fn new(url: Url) -> Result<Self, anyhow::Error> {
        gstreamer::init()?;

        let playbin = gstreamer::ElementFactory::make("playbin")
            .name("player")
            .property("uri", url.as_str())
            .build()?;

        let flags = playbin.property_value("flags");
        let flags_class = glib::FlagsClass::with_type(flags.type_()).unwrap();

        let flags = flags_class
            .builder_with_value(flags)
            .unwrap()
            .set_by_nick("audio")
            .unset_by_nick("video")
            .unset_by_nick("text")
            .build()
            .unwrap();

        playbin.set_property_from_value("flags", &flags);

        Ok(Self {
            playbin,
            duration: None,
        })
    }

    pub fn play(&mut self) -> Result<(), anyhow::Error> {
        self.playbin
            .set_state(gstreamer::State::Playing)
            .context("Failed to set playing state")?;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), anyhow::Error> {
        self.playbin
            .set_state(gstreamer::State::Paused)
            .context("Failed to set paused state")?;
        Ok(())
    }

    pub fn toggle_playing(&mut self) -> Result<(), anyhow::Error> {
        if self.is_playing() {
            self.pause()
        } else {
            self.play()
        }
    }

    pub fn is_playing(&self) -> bool {
        self.playbin.current_state() == gstreamer::State::Playing
    }

    pub fn current_position(&self) -> Duration {
        let position = self
            .playbin
            .query_position::<gstreamer::ClockTime>()
            .unwrap_or_default();
        position.into()
    }

    pub fn duration(&mut self) -> Duration {
        if let Some(duration) = self.duration {
            return duration;
        }

        let Some(duration) = self.playbin.query_duration::<gstreamer::ClockTime>() else {
            return Default::default();
        };

        let duration = duration.into();
        self.duration.replace(duration);
        duration
    }

    pub fn seek_to(&self, duration: Duration) -> Result<(), anyhow::Error> {
        self.playbin
            .seek_simple(
                gstreamer::SeekFlags::FLUSH,
                ClockTime::from_nseconds(duration.as_nanos() as u64),
            )
            .context("Could not seek to position")?;
        Ok(())
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        _ = self.playbin.set_state(gstreamer::State::Null);
    }
}
