use std::env;
use std::fs::File;

use anyhow::Result;
use image::codecs::gif::{GifEncoder, Repeat};
use image::io::Reader;
use image::Frame;

fn main() -> Result<()> {
    let mut paths: Vec<_> = env::args().skip(1).collect();
    paths.sort();

    let mut out = GifEncoder::new_with_speed(File::create("anime.gif")?, 5);
    out.set_repeat(Repeat::Infinite)?;
    for p in paths {
        let img = Reader::open(p)?.decode()?;
        out.encode_frame(Frame::new(img.into_rgba8()))?;
    }

    Ok(())
}
