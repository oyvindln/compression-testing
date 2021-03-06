use std::io::{self, Read};
use std::time::Instant;

use shared::{Wrapper, BenchResult};


fn decompress_inflate(data: &[u8], wrapper: Wrapper) -> io::Result<usize> {
    use inflate::{inflate_bytes, inflate_bytes_zlib};
    match wrapper {
        Wrapper::None => inflate_bytes(data).map(|r| r.len()),
        Wrapper::Zlib => inflate_bytes_zlib(data).map(|r| r.len()),
        Wrapper::Gzip => Err("Not supported!".to_owned()),
    }.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn decompress_flate2(data: &[u8], wrapper: Wrapper) -> io::Result<usize> {
    use flate2::read::{DeflateDecoder, ZlibDecoder, GzDecoder};

    let mut result = Vec::new();

    match wrapper {
        Wrapper::None => DeflateDecoder::new(data).read_to_end(&mut result),
        Wrapper::Zlib => ZlibDecoder::new(data).read_to_end(&mut result),
        Wrapper::Gzip => GzDecoder::new(data).read_to_end(&mut result),
    }
}

fn decompress_libflate(data: &[u8], wrapper: Wrapper) -> io::Result<usize> {
    use libflate;

    let mut result = Vec::new();

    match wrapper {
        Wrapper::None => libflate::deflate::Decoder::new(data).read_to_end(&mut result),
        Wrapper::Zlib => {
            libflate::zlib::Decoder::new(data).unwrap().read_to_end(
                &mut result,
            )
        }
        Wrapper::Gzip => {
            libflate::gzip::Decoder::new(data).unwrap().read_to_end(
                &mut result,
            )
        }
    }
}

fn decompress_miniz_oxide(data: &[u8], wrapper: Wrapper) -> io::Result<usize> {
    use miniz_oxide;

    match wrapper {
        Wrapper::None => miniz_oxide::inflate::decompress_to_vec(data).map_err(|_| "Failed".to_owned()),
        Wrapper::Zlib => miniz_oxide::inflate::decompress_to_vec_zlib(data).map_err(|_| "Failed".to_owned()),
        Wrapper::Gzip => Err("Not supported!".to_owned())
    }.map(|r| r.len()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn time_func<F>(data: &[u8], decoder: &str, wrapper: Wrapper, func: F) -> BenchResult
where
    F: Fn(&[u8], Wrapper) -> io::Result<usize>,
{
    let start = Instant::now();
    let res = func(data,wrapper);
    let mut time = start.elapsed();

    for _ in 0..2 {
        let start = Instant::now();
        let _ = func(data,wrapper);
        time += start.elapsed();
    }

    BenchResult::from_result(decoder,res.map(|o| o),time / 3)
}

pub fn time_decompress(data: &[u8], wrapper: Wrapper) {
    let mut results = [
        time_func(data, "Inflate", wrapper, decompress_inflate),
        time_func(data, "flate2", wrapper, decompress_flate2),
        time_func(data, "libflate", wrapper, decompress_libflate),
        time_func(data, "miniz_oxide", wrapper, decompress_miniz_oxide),
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
