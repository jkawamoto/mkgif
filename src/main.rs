use std::fs::File;
use std::string::ToString;

use anyhow::{bail, Result};
use clap::Parser;
use image::codecs::gif::{GifEncoder, Repeat};
use image::io::Reader;
use image::Frame;
use indicatif::ProgressBar;

const DEFAULT_OUTPUT: &str = "output.gif";

/// Create an animation GIF from the given image files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the output file [default: output.gif].
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,
    /// Processing speed in [1, 30]. The higher the value the faster it runs at the cost of image quality.
    #[arg(short, long, value_name = "SPEED", default_value_t = 10)]
    speed: i32,
    /// Paths to the input image files.
    paths: Vec<String>,
}

fn main() -> Result<()> {
    let mut args = Args::parse();
    if args.paths.is_empty() {
        bail!("no image files are given");
    }
    args.paths.sort();

    let pb = ProgressBar::new(args.paths.len() as u64);
    let mut out = GifEncoder::new_with_speed(
        File::create(args.output.unwrap_or(DEFAULT_OUTPUT.to_string()))?,
        args.speed,
    );
    out.set_repeat(Repeat::Infinite)?;
    for p in args.paths {
        let img = Reader::open(p)?.decode()?;
        out.encode_frame(Frame::new(img.into_rgba8()))?;
        pb.inc(1);
    }
    pb.finish();

    Ok(())
}
