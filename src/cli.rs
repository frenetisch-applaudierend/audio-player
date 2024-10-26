use std::path::PathBuf;

use url::Url;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The audio file to play
    pub media: String,
}

impl Cli {
    pub fn url(&self) -> Option<Url> {
        if let Ok(url) = Url::parse(&self.media) {
            return Some(url);
        }

        let path = PathBuf::from(&self.media).canonicalize().ok()?;
        Url::from_file_path(path).ok()
    }
}
