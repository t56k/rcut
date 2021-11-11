use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use ffmpeg_next::*;
use rand::Rng;

use super::errors::ReadError;

#[derive(Debug)]
pub struct CollectionOptions {
    pub number_of_files: i16,
    pub files: Vec<FileCut>,
}

#[derive(Debug)]
pub struct FileCut {
    pub path: DirEntry,
    pub start: f64,
    pub stop: f64,
}

pub fn get_files(in_dir: &Path, filetype: String, number_of_files: i16, length: f64) -> Result<CollectionOptions, ReadError> {
    let mut dir_entries: Vec<PathBuf> = vec![in_dir.to_path_buf()];
    let mut files_to_cut: Vec<FileCut> = vec![];

    while let Some(entry) = dir_entries.pop() {
        for inner_entry in fs::read_dir(&entry)? {
            if let Ok(entry) = inner_entry {
                if entry.path().is_dir() {
                    dir_entries.push(entry.path());
                } else {
                    if entry.path().extension() == Some(OsStr::new(&filetype)) {
                        files_to_cut.push(init_file_cut(entry, length));
                    }
                }
            }
        }
    }

    Ok(CollectionOptions {
        number_of_files,
        files: files_to_cut
    })
}

fn init_file_cut(path: DirEntry, length: f64) -> FileCut {
    let mut duration: f64;
    match ffmpeg_next::format::input(&path) {
        Ok(context) => {
            duration = context.duration() as f64;
        }
        Err(error) => println!("error: {}", error)
    }
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();

    FileCut {
        path,
        start: (duration * x) - length,
        stop: (duration * x),
    }
}
