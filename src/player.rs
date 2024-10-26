use std::time::Duration;

use gstreamer::prelude::{ElementExt, ElementExtManual, ObjectExt};
use url::Url;

pub struct Player {
    playbin: gstreamer::Element,
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

        Ok(Self { playbin })
    }

    pub fn play(&mut self) -> Result<(), anyhow::Error> {
        self.playbin.set_state(gstreamer::State::Playing)?;
        Ok(())
    }

    pub fn current_position(&self) -> Duration {
        let position = self
            .playbin
            .query_position::<gstreamer::ClockTime>()
            .unwrap_or_default();
        position.into()
    }

    pub fn is_playing(&self) -> bool {
        let state = self.playbin.current_state();
        println!("Current state: {:?}", state);
        state == gstreamer::State::Playing
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        _ = self.playbin.set_state(gstreamer::State::Null);
    }
}
