use std::env;
use std::fs;
use std::io;
use untitled2::{decompress_by_zstd, compress_by_zstd};

const SUFFIX: &'static str = ".zst";

fn main() {
    for arg in env::args().skip(1) {
        if arg.ends_with(SUFFIX) {
            match decompress_by_zstd(&arg) {
                Ok(()) => println!("Decompressed {}", arg),
                Err(e) => println!("Error decompressing {}: {}", arg, e),
            }
        } else {
            match compress_by_zstd(&arg) {
                Ok(()) => println!("Compressed {}", arg),
                Err(e) => println!("Error compressing {}: {}", arg, e),
            }
        }
    }
}
