use zstd;

use std::env;
use std::fs;
use std::io;

const SUFFIX: &'static str = ".zst";

fn main() {
    for arg in env::args().skip(1) {
        if arg.ends_with(SUFFIX) {
            match decompress(&arg) {
                Ok(()) => println!("Decompressed {}", arg),
                Err(e) => println!("Error decompressing {}: {}", arg, e),
            }
        } else {
            match compress(&arg) {
                Ok(()) => println!("Compressed {}", arg),
                Err(e) => println!("Error compressing {}: {}", arg, e),
            }
        }
    }
}

fn compress(source: &str) -> io::Result<()> {
    let mut file = fs::File::open(source)?;
    let mut encoder = {
        let target = fs::File::create(source.to_string() + SUFFIX)?;
        zstd::Encoder::new(target, 1)?
    };

    io::copy(&mut file, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

fn decompress(source: &str) -> io::Result<()> {
    let mut decoder = {
        //TODO dir 안에 전부 (복사하면 크니까 현재 받아놓은 경로로)
        let file = fs::File::open(source)?;
        zstd::Decoder::new(file)?
    };

    let split = source.split("/");
    let vec = split.collect::<Vec<&str>>();

    let mut dest = String::from("./output/");
    dest.push_str(vec[vec.len()-1]);

    println!("dest: {}", dest);
    let mut target = fs::File::create(dest.trim_end_matches(SUFFIX))?;

    io::copy(&mut decoder, &mut target)?;

    Ok(())
}

fn read_dir() {
    if let Ok(entries) = fs::read_dir("./asset/mysqlbinlog") {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                println!("{:?}", entry.file_name());
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn visit_dirs() {
        read_dir();
    }

    #[test]
    fn decompress_41gb_files() {
        if let Ok(entries) = fs::read_dir("./asset/mysqlbinlog") {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.
                    println!("{:?}", entry.file_name());

                    let mut path = "./asset/mysqlbinlog/".to_string();
                    println!("path1: {}", path);

                    match entry.file_name().to_str() {
                        Some(file_name) => {
                            path.push_str(file_name);
                            println!("path2: {}", path);
                            decompress(path.as_ref());
                        },
                        None => (),
                    }
                }
            }
        }
    }
}