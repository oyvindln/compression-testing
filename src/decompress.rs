use std::io::Read;
use std::time::Instant;

use shared::{Wrapper, BenchResult, UsedCrate};


fn decompress_inflate(data: &[u8], wrapper: Wrapper) -> Option<Vec<u8>> {
    use inflate::{inflate_bytes,inflate_bytes_zlib};
    match wrapper {
        Wrapper::None => {
            inflate_bytes(data).ok()
        },
        Wrapper::Zlib => {
            inflate_bytes_zlib(data).ok()
        },
        Wrapper::Gzip => {
            None
        }
    }
}

fn decompress_flate2(data: &[u8], wrapper: Wrapper) -> Option<Vec<u8>> {
    use flate2::read::{DeflateDecoder,ZlibDecoder,GzDecoder};

    let mut result = Vec::new();

    let res = match wrapper {
        Wrapper::None => {
            DeflateDecoder::new(data).read_to_end(&mut result)
        },
        Wrapper::Zlib => {
            ZlibDecoder::new(data).read_to_end(&mut result)
        },
        Wrapper::Gzip => {
            GzDecoder::new(data).unwrap().read_to_end(&mut result)
        }
    };

    if res.is_ok() {
        Some(result)
    } else {
        None
    }
}

fn decompress_libflate(data: &[u8], wrapper: Wrapper) -> Option<Vec<u8>> {
    use libflate;

    let mut result = Vec::new();

    let res = match wrapper {
        Wrapper::None => {
            libflate::deflate::Decoder::new(data).read_to_end(&mut result)
        },
        Wrapper::Zlib => {
            libflate::zlib::Decoder::new(data).unwrap().read_to_end(&mut result)
        },
        Wrapper::Gzip => {
            libflate::gzip::Decoder::new(data).unwrap().read_to_end(&mut result)
        }
    };

    if res.is_ok() {
        Some(result)
    } else {
        None
    }
}

fn time_decoder<F>(data: &[u8], decoder: UsedCrate, wrapper: Wrapper, func: F) -> BenchResult
    where F: Fn(&[u8],Wrapper) -> Option<Vec<u8>> {

    let start = Instant::now();
    let res = func(data,wrapper);
    let time = start.elapsed();
    BenchResult::from_result(decoder,res.map(|o| o.len()),time)
}

pub fn time_decompress(data: &[u8],wrapper: Wrapper) {
    let mut results = [time_decoder(data,UsedCrate::DeflateInflate,wrapper,decompress_inflate),
                       time_decoder(data,UsedCrate::Flate2,wrapper,decompress_flate2),
                       time_decoder(data,UsedCrate::LibFlate,wrapper,decompress_libflate)];

    results.sort();

    println!("Wrapper: {:?}", wrapper);
    for r in results.iter() {
        if let Some(size) = r.size {
            println!("{:?} - {:?} - (size: {})", r.library, r.time_used.unwrap(), size);
        } else {
            println!("{:?} - failed", r.library);
        }
    }
}
