use std::path::PathBuf;
use structopt::StructOpt;

mod collection;
mod ffmpeg;

use crate::collection::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "rcut", about = "Get random cuts from media files")]
struct Opt {
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    in_dir: PathBuf,
    #[structopt(short = "t", long = "filetype", default_value = "mkv")]
    filetype: String,
    #[structopt(short = "s", long = "samples", default_value = "1")]
    samples: i16,
    #[structopt(short = "l", long = "length", default_value = "1")]
    length: f64,
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    out_dir: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let mut collection =
        select(&opt.in_dir, opt.filetype, opt.samples, opt.length).unwrap();

    for file in collection.files.iter_mut() {
        cut_file(file, &opt.out_dir);
    }

    println!("cut {:?}", collection);
}
