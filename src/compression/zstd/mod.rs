use zstd;

use std::fs;
use std::io;

const SUFFIX: &'static str = ".zst";

pub fn compress(source: &str) -> io::Result<()> {
    let mut file = fs::File::open(source)?;
    let mut encoder = {
        let target = fs::File::create(source.to_string() + SUFFIX)?;
        zstd::Encoder::new(target, 1)?
    };

    io::copy(&mut file, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

pub fn decompress(source: &str) -> io::Result<()> {
    let mut decoder = {
        let file = fs::File::open(source)?;
        zstd::Decoder::new(file)?
    };

    let split = source.split("/");
    let vec = split.collect::<Vec<&str>>();
    println!("cur_dir: {:?}", std::env::current_dir());

    let mut dest = String::from("./output/");
    dest.push_str(vec[vec.len()-1]);
    println!("dest: {}", dest);

    let mut target = fs::File::create(dest.trim_end_matches(SUFFIX))?;

    io::copy(&mut decoder, &mut target)?;

    Ok(())
}



mod tests {
    use super::*;

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