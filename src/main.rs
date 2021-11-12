use std::path::PathBuf;
use structopt::StructOpt;

mod collection;
mod error;
mod ffmpeg;

use crate::collection::get_files;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rcut",
    about = "Get random cuts from media files"
)]
struct Opt {
    #[structopt(
        short = "i",
        long = "input",
        parse(from_os_str)
    )]
    in_dir: PathBuf,
    #[structopt(
        short = "t",
        long = "filetype",
        default_value = "mkv"
    )]
    filetype: String,
    #[structopt(
        short = "n",
        long = "number",
        default_value = "99"
    )]
    number_of_files: i16,
    #[structopt(
        short = "l",
        long = "length",
        default_value = "1"
    )]
    length: f64,
    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str)
    )]
    out_dir: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let files = get_files(&opt.in_dir, opt.filetype, opt.number_of_files, opt.length);

    println!("{:?}", files);
}
