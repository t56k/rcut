use std::borrow::Cow;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn get_video_duration(input: &str) -> Result<f64, std::io::Error> {
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
        .expect("couldnt get duration");

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let out = stdout_reader
        .lines()
        .next()
        .expect("cannot read lines")
        .unwrap()
        .parse::<f64>()
        .expect("cannot parse int");

    cmd.wait().expect("failed duration");
    Ok(out)
}

pub fn extract_audio_clip(input: &str, start: &str, stop: &str, output: &str) {
    let filename = basename(input, '/');
    let mut cmd = Command::new("ffmpeg")
        .args(&[
            "-i",
            input,
            "-ss",
            start,
            "-t",
            stop,
            &(output.to_owned() + "/" + &filename + "-" + start + ".mp3"),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("couldnt win");

    cmd.wait().expect("failed extraction");
}

fn basename<'a>(path: &'a str, sep: char) -> Cow<'a, str> {
    let mut pieces = path.rsplit(sep);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}
