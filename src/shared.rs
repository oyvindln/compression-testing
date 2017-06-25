use std::time::Duration;
use std::convert::From;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Wrapper {
    None,
    Zlib,
    Gzip
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum UsedCrate {
    Flate2,
    LibFlate,
    DeflateInflate,
}

impl From<UsedCrate> for String {
    fn from(c: UsedCrate) -> String {
        match c {
            UsedCrate::Flate2 => "Flate2",
            UsedCrate::LibFlate => "LibFlate",
            UsedCrate::DeflateInflate => "Deflate or Inflate"
        }.into()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub struct BenchResult {
    pub time_used: Option<Duration>,
    pub size: Option<usize>,
    pub init_time: Option<Duration>,
    pub library: UsedCrate,
}

impl BenchResult {
    pub fn new(library: UsedCrate) -> BenchResult {
        BenchResult{
            library: library,
            size: None,
            time_used: None,
            init_time: None,
        }
    }

    pub fn from_result(library: UsedCrate, size: Option<usize>, time_used: Duration) -> BenchResult {
        BenchResult{
            library: library,
            size: size,
            time_used: Some(time_used),
            init_time: None,
        }
    }
}
