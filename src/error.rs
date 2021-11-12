use snafu::Snafu;
use std::io;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("Failed to spawn ffmpeg. {}", source))]
    CommandSpawnError { source: io::Error },

    #[snafu(display("FFMPEG failed to exit properly. {}", source))]
    FFMPEGExitError { source: io::Error },

    #[snafu(display("Failed to read dimensions from ffmpeg stdout."))]
    FFMPEGLineReadError,

    #[snafu(display("Could not parse video duration. {}", source))]
    ParseDurationError { source: std::num::ParseFloatError },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
