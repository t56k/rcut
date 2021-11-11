use snafu::{OptionExt, ResultExt};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::error::*;

pub fn get_video_duration(input_video_path: &str) -> Result<f64> {
    let mut cmd = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            input_video_path,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .context(CommandSpawnError)?;

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let duration = stdout_reader
        .lines()
        .next()
        .context(FFMPEGLineReadError)?
        .unwrap()
        .parse::<f64>()
        .context(ParseDurationError)?;

    cmd.wait().context(FFMPEGExitError)?;

    Ok(duration)
}

// pub fn extract_clip(input_video_path: &str) -> Result<str> {
//     let mut cmd = Command::new("ffmpeg")
//         .args(&[
//             "-i",
//             input_video_path,
//         ])
//         .stdout(Stdio::piped())
//         .spawn()
//         .context(CommandSpawnError)?;
// }
