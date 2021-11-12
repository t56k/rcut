use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use rand::Rng;

use crate::error::*;
use crate::ffmpeg::*;

#[derive(Debug)]
pub struct CollectionOptions<'a> {
    pub files: Vec<FileCut<'a>>,
}

#[derive(Debug)]
pub struct FileCut<'a> {
    pub path: &'a str,
    pub duration: f64,
    pub start: f64,
    pub stop: f64,
}

pub fn select(
    in_dir: &Path,
    filetype: String,
    length: f64,
) -> Result<CollectionOptions, std::io::Error> {
    let mut dir_entries: Vec<PathBuf> = vec![in_dir.to_path_buf()];
    let mut files_to_cut: Vec<FileCut> = vec![];

    while let Some(entry) = dir_entries.pop() {
        for inner_entry in fs::read_dir(&entry)? {
            if let Ok(entry) = inner_entry {
                if entry.path().extension() == Some(OsStr::new(&filetype)) {
                    let path = string_to_static_str(
                        entry.path().into_os_string().into_string().unwrap(),
                    );
                    files_to_cut.push(init_file_cut(path, length));
                }
            }
        }
    }

    Ok(CollectionOptions {
        files: files_to_cut,
    })
}

pub fn init_file_cut(path: &str, length: f64) -> FileCut {
    let duration: f64 = get_video_duration(path).unwrap();
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();

    FileCut {
        path,
        duration,
        start: (duration * x) - length,
        stop: length,
    }
}

pub fn cut_file(file: &FileCut, out_dir: &Path) {
    extract_audio_clip(
        file.path,
        &file.start.to_string(),
        &file.stop.to_string(),
        &out_dir
            .to_path_buf()
            .into_os_string()
            .into_string()
            .unwrap(),
    );
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
