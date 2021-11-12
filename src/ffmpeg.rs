use snafu::{OptionExt, ResultExt};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::error::*;

pub fn get_video_duration(input: &str) -> Result<f64> {
    let mut cmd = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            input,
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

pub fn extract_audio_clip(input: &str, start: &str, stop: &str, output: &str) -> Result<f64> {
    let split_input = input.split(".").collect::<String>();
    let mut cmd = Command::new("ffmpeg")
        .args(&[
            "-i",
            input,
            "-ss",
            start,
            "-t",
            stop,
            &(output.to_owned() + "/" + &split_input + "-" + start + ".mp3"),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .context(CommandSpawnError)?;

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let out = stdout_reader
        .lines()
        .next()
        .context(FFMPEGLineReadError)?
        .unwrap()
        .parse::<f64>()
        .context(ParseDurationError)?;

    cmd.wait().context(FFMPEGExitError)?;
    Ok(out)
}
