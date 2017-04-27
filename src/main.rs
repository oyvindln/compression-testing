extern crate deflate;
extern crate flate2;
extern crate byteorder;
extern crate inflate;
extern crate clap;
extern crate term;

#[macro_use] extern crate pretty_assertions;

use std::{io, fs};
use std::path::Path;
use clap::{App, Arg};
use std::time::Instant;

use byteorder::BigEndian;
use inflate::InflateStream;
use flate2::Compression;

fn get_file_data(name: &Path) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;
    let mut input = Vec::new();
    match File::open(name) {
        Ok(mut f) => {
            f.read_to_end(&mut input).unwrap();
            input
        },
        Err(e) => {
            println!("ERROR: FAILED TO OPEN: {:?}", name);
            panic!("{}", e);
        }
    }
}

/// Helper function to decompress into a `Vec<u8>`
fn decompress_to_end(input: &[u8]) -> Vec<u8> {
    use std::str;

    {
    let mut inflater = InflateStream::from_zlib();
    let mut out = Vec::<u8>::new();
    let mut n = 0;
    // println!("input len {}", input.len());
    while n < input.len() {
        let res = inflater.update(&input[n..]);
        if let Ok((num_bytes_read, result)) = res {
            // println!("result len {}, bytes_read {}", result.len(), num_bytes_read);
            n += num_bytes_read;
            out.extend(result);
        } else {
            //println!("Output: `{}`", str::from_utf8(&out).unwrap());
            println!("Output decompressed: {}", out.len());
            res.unwrap();
        }

    }
    out
    }
/*
    use std::io::Read;
    use flate2::read::ZlibDecoder;
    let mut result = Vec::new();
    let mut e = ZlibDecoder::new(input);

    let res = e.read_to_end(&mut result);
    if let Ok(_) = res {
        // println!("{} bytes read successfully", n);
    } else {
        println!("ERROR: Failed to decompress! result size: {}", result.len());
        res.unwrap();
    }
    result*/
}

fn write_data(file_name: &str, data: &[u8]) {
    use std::fs::File;
    use std::io::Write;
    println!("Writing to: {}", file_name);
    let mut f = File::create(file_name).unwrap();
    f.write_all(&data).unwrap();
}

fn get_adler32(input: &[u8]) -> u32 {
    use byteorder::ByteOrder;
    let last_bytes = &input[input.len() - 4..];
    BigEndian::read_u32(last_bytes)
}

fn _print_runs(input: &[u8]) {
    let mut last_b = input[0];
    let mut counter = 0;
    for &i in &input[1..] {
        if i == last_b {
            counter += 1;
        } else {
            if counter > 3 {
                println!("Run of {} copies of byte `{}` ", counter, last_b);
            }
            last_b = i;
            counter = 0;
        }
    }
}

fn _test_flush(data: &[u8]) {
    use flate2::{Compress, Compression, Flush, Status};
    let mut c = Compress::new(Compression::Default, true);
    let mut v = Vec::with_capacity(data.len());
    let s = c.compress_vec(data, &mut v, Flush::Sync);
    println!("Status is:");
    match s {
        Status::Ok => println!("Status OK"),
        Status::BufError => println!("Buffer error"),
        Status::StreamEnd => println!("Stream end"),
    }
    // println!("Status: {}", s);
    write_data("flush_test.deflate", &v);
}

fn _test_inflate() {
    for i in 0..69000 {
        let test = vec![22; i];
        // test[32768] = 5;
        let compr = deflate::deflate_bytes_zlib(&test);
        let l = decompress_to_end(&compr);
        assert!(l[..] == test[..]);
    }
}

fn main() {
    let matches = App::new("Compression tester")
        .arg(Arg::with_name("PATH")
            .required(true)
            .index(1)
            .takes_value(true))
        .arg(Arg::with_name("write").short("w").long("write"))
        .get_matches();

    let path = Path::new(matches.value_of("PATH").unwrap());
    let write = matches.is_present("write");

    if path.is_file() {
        let _ = test_file(path, write);
    } else if path.is_dir() {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::BRIGHT_GREEN).unwrap_or_default();
        write!(t, "\tTesting files in dir: ").unwrap();
        t.reset().unwrap();
        writeln!(t, " {:?} ", path).unwrap();
        drop(t);
        visit_dirs(path, write, &test_file).unwrap();
    } else {
        println!("Unknown path!");
    }
}

fn visit_dirs(dir: &Path, write: bool, cb: &Fn(&Path, bool) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            if entry.is_err() {
                continue
            };
            let entry = try!(entry);
            let path = entry.path();
            if path.is_dir() {
                let _ = (visit_dirs(&path, write, cb));
            } else {
                cb(&entry.path(), write)?;
            }
        }
    }
    Ok(())
}

fn test_file(path: &Path, write: bool) -> io::Result<()> {
    use std::io::Write;
    use term::Attr;
    use term::color;

    let mut t = term::stdout().unwrap();

    let data = get_file_data(path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    t.fg(color::BRIGHT_GREEN).unwrap_or_default();
    write!(t, "\tTesting file:")?;
    t.reset()?;
    t.attr(Attr::Bold).unwrap_or_default();
    write!(t, " {:?} ", path)?;
    writeln!(t, "Input size: {}", data.len())?;
    t.reset()?;

    let flate_t;
    let deflate_t;
    let flate2_size;
    let deflate_size;

    {
        // test_flush(&data);
        // print_runs(&data);
        // test_inflate();
    }

    {
        print!("Flate2: ");
        let start = Instant::now();
        let flate2_compressed =
            let mut e = flate2::write::ZlibEncoder::new(Vec::new(), Compression::Default);
            e.write_all(&data).unwrap();
            e.finish().unwrap()
        };

        flate_t = start.elapsed();

        println!("Time elapsed: {:?}", flate_t);

        println!("Compressed size: {}, Adler32: {}",
                 flate2_compressed.len(),
                 get_adler32(&flate2_compressed));
        flate2_size = flate2_compressed.len();
        if write {
            write_data(&format!("{}.flate2", file_name), &flate2_compressed);
        }
    }

    println!("-");

    {
        print!("Deflate: ");
        let start = Instant::now();
        let compressed_deflate = deflate::deflate_bytes_zlib_conf(&data,
                                                                  deflate::Compression::Default);

        deflate_t = start.elapsed();

        println!("Time elapsed: {:?}", deflate_t);

        println!("Compressed size: {}, Adler32: {}",
                 compressed_deflate.len(),
                 get_adler32(&compressed_deflate));

        deflate_size = compressed_deflate.len();

        if write {
            write_data(&format!("{}.deflate", file_name), &compressed_deflate);
        }

        let decompressed = decompress_to_end(&compressed_deflate);
        for (n, (&orig, &dec)) in data.iter().zip(decompressed.iter()).enumerate() {
            if orig != dec {
                println!("Byte at {} differs: orig: {}, dec: {}", n, orig, dec);
                println!("Original: {:?}, decoded: {:?}", &data[n..n + 5], &decompressed[n..n+5]);
                break;
            }
        }

        assert_eq!(data.len(), decompressed.len());

        assert!(data == decompressed);
    }

    println!("-");

    write!(t, "Time difference: ")?;

    if deflate_t > flate_t {
        t.fg(color::BRIGHT_RED).unwrap_or_default();
        writeln!(t, "Flate faster: {:?}", deflate_t - flate_t)?;
    } else if deflate_t < flate_t {
        t.fg(color::BRIGHT_GREEN).unwrap_or_default();
        writeln!(t, "Deflate faster: {:?}", flate_t - deflate_t)?;
    } else {
        t.fg(color::YELLOW).unwrap_or_default();
        writeln!(t, "None")?;
    };
    t.reset()?;

    write!(t, "Size difference: ")?;
    let diff = flate2_size as i64 - deflate_size as i64;
    if diff > 0 {
        t.fg(color::BRIGHT_GREEN).unwrap_or_default();
    } else if diff < 0 {
        t.fg(color::BRIGHT_RED).unwrap_or_default();
    } else {
        t.fg(color::YELLOW).unwrap_or_default();
    };
    writeln!(t, "{}", -diff)?;

    t.reset()?;

    println!("-------------------------------------------");
    Ok(())
}
