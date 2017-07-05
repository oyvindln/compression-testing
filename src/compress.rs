use std::io::{self, Write};
use std::time::Instant;

use shared::*;

fn time_func<F>(data: &[u8], decoder: &str, wrapper: Wrapper, level: Level, func: F) -> BenchResult
where
    F: Fn(&[u8], Wrapper, Level) -> io::Result<usize>,
{

    let start = Instant::now();
    let res = func(data, wrapper, level);
    let time = start.elapsed();
    BenchResult::from_result(decoder, res, time)
}


fn compress_flate2(data: &[u8], wrapper: Wrapper, level: Level) -> io::Result<usize> {
    use flate2::write::*;
    let vec = Vec::with_capacity(data.len() / 3);
    match wrapper {
        Wrapper::Gzip => {
            let mut e = GzEncoder::new(vec, level.into());
            e.write_all(data)?;
            e.finish()
        }
        Wrapper::Zlib => {
            let mut e = ZlibEncoder::new(vec, level.into());
            e.write_all(data)?;
            e.finish()
        }
        Wrapper::None => {
            let mut e = DeflateEncoder::new(vec, level.into());
            e.write_all(data)?;
            e.finish()
        }
    }.map(|a| a.len())
}

fn compress_deflate(data: &[u8], wrapper: Wrapper, level: Level) -> io::Result<usize> {
    use deflate::write::*;
    use deflate::CompressionOptions;

    let level = CompressionOptions::from(level);
    let vec = Vec::with_capacity(data.len() / 3);
    match wrapper {
        Wrapper::Gzip => {
            let mut e = GzEncoder::new(vec, level);
            e.write_all(data)?;
            e.finish()
        }
        Wrapper::Zlib => {
            let mut e = ZlibEncoder::new(vec, level);
            e.write_all(data)?;
            e.finish()
        }
        Wrapper::None => {
            let mut e = DeflateEncoder::new(vec, level);
            e.write_all(data)?;
            e.finish()
        }
    }.map(|a| a.len())
}

fn compress_libflate(data: &[u8], wrapper: Wrapper, _: Level) -> io::Result<usize> {
    // Similar block length to miniz and deflate
    // The default is very large and probably not optimal.
    let block_length = 1024 * 31;
    use libflate::{gzip, zlib, deflate};
    let vec = Vec::with_capacity(data.len() / 3);
    match wrapper {
        Wrapper::None => {
            let opts = deflate::EncodeOptions::new().block_size(block_length);
            let mut enc = deflate::Encoder::with_options(vec, opts);
            enc.write_all(data)?;
            enc.finish().into_result()
        }
        Wrapper::Zlib => {
            let opts = zlib::EncodeOptions::new().block_size(block_length);
            let mut enc = zlib::Encoder::with_options(vec, opts)?;
            enc.write_all(data)?;
            enc.finish().into_result()
        }
        Wrapper::Gzip => {
            let opts = gzip::EncodeOptions::new().block_size(block_length);
            let mut enc = gzip::Encoder::with_options(vec, opts)?;
            enc.write_all(data)?;
            enc.finish().into_result()
        }
    }.map(|a| a.len())
}


pub fn time_compress(data: &[u8], wrapper: Wrapper, level: Level) {
    let mut results = [
        time_func(data, "deflate", wrapper, level, compress_deflate),
        time_func(data, "flate2", wrapper, level, compress_flate2),
        time_func(data, "libflate", wrapper, level, compress_libflate),
    ];

    results.sort_by(|lhs, rhs| lhs.time_used.cmp(&rhs.time_used));

    println!("Wrapper: {:?}", wrapper);
    for r in results.iter() {
        if let Ok(size) = r.size {
            println!(
                "{} - {:?} - (size: {})",
                r.library,
                r.time_used.unwrap(),
                size
            );
        } else {
            println!("{} - failed: {}", r.library, r.size.as_ref().unwrap_err());
        }
    }
}
