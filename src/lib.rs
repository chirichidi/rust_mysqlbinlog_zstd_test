mod compression;

use std::io;

pub fn compress_by_zstd(source: &str) -> io::Result<()> {
    compression::zstd::compress(source)
}

pub fn decompress_by_zstd(source: &str) -> io::Result<()> {
    compression::zstd::decompress(source)
}
