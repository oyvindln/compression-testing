# compression-testing

Tool written origintally to test the [deflate](https://crates.io/crates/deflate), but now has functionality to compare several crates that do deflate compression/decompression.

## Usage: 
### Compare compression efficiency and speed of raw deflate encoding using the default compression level:
`cargo run --release -- -c /path/to/directory/or/file/to/test`

### Compare decompression speed
`cargo run --release -- -d /path/to/directory/or/file/to/test`

Additionally, the `-t <type>` switch, where type is either `zlib` or `gzip` can be used to compare compression/decompression with a zlib or gzip wrapper respectively.
