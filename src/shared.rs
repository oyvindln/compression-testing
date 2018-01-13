use std::time::Duration;
use std::convert::From;
use std::io;

use flate2::Compression;
use deflate::CompressionOptions;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Wrapper {
    None,
    Zlib,
    Gzip,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UsedCrate {
    Flate2,
    LibFlate,
    DeflateInflate,
}

#[derive(Copy, Clone, Debug)]
pub enum Level {
    Best = 9,
    Default = 6,
    Fast = 1,
}

impl From<Level> for CompressionOptions {
    fn from(compression: Level) -> CompressionOptions {
        match compression {
            Level::Fast => CompressionOptions::fast(),
            //Level::Fast => CompressionOptions::rle(),
            Level::Default => CompressionOptions::default(),
            Level::Best => CompressionOptions::high(),
        }
    }
}

impl From<Level> for Compression {
    fn from(compression: Level) -> Compression {
        match compression {
            Level::Fast => Compression::fast(),
            Level::Default => Compression::default(),
            Level::Best => Compression::best(),
        }
    }
}

impl From<UsedCrate> for String {
    fn from(c: UsedCrate) -> String {
        match c {
            UsedCrate::Flate2 => "Flate2",
            UsedCrate::LibFlate => "LibFlate",
            UsedCrate::DeflateInflate => "Deflate or Inflate",
        }.into()
    }
}

#[derive(Debug)]
pub struct BenchResult {
    pub time_used: Option<Duration>,
    pub size: io::Result<usize>,
    pub init_time: Option<Duration>,
    pub library: String,
}

impl BenchResult {
    pub fn from_result(library: &str, size: io::Result<usize>, time_used: Duration) -> BenchResult {
        BenchResult {
            library: library.to_owned(),
            size: size,
            time_used: Some(time_used),
            init_time: None,
        }
    }
}
