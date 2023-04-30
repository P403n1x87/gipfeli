use brotli::Decompressor;
use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use glob::glob;
use std::{
    fs::File,
    io::{Cursor, Read, Write},
};

/// Quickly share small files via text messages
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of file patterns to match for packing
    #[arg(num_args(0..))]
    patterns: Vec<String>,
}

fn compress(input: &Vec<u8>) -> Vec<u8> {
    let mut writer = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
    writer.write_all(&input).unwrap();
    let compressed = writer.into_inner();

    compressed
}

fn pack(paths: Vec<String>) -> String {
    let mut ar = tar::Builder::new(Vec::new());

    for pattern in paths {
        let result = glob(&pattern).unwrap();
        for entry in result {
            match entry {
                Ok(path) => {
                    println!("packing {:?}", path.as_path());
                    ar.append_path(&path).unwrap();
                }
                Err(e) => {
                    println!("error: {}", e);
                    continue;
                }
            }
        }
    }

    let data = ar.into_inner().unwrap();

    z85::encode(compress(&data))
}

pub fn decompress(input: Vec<u8>) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut decompressor = Decompressor::new(Cursor::new(input), 4096);

    decompressor.read_to_end(&mut decompressed).unwrap();

    decompressed
}

fn unpack(data: &String) -> () {
    let data = decompress(z85::decode(data).unwrap());

    let mut archive = tar::Archive::new(Cursor::new(data));
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let path = entry.path().unwrap();
        let mut file = File::create(path).unwrap();

        std::io::copy(&mut entry, &mut file).unwrap();

        println!("unpacked {:?}", entry.path().unwrap());
    }
}

fn main() {
    let args = Args::parse();
    if args.patterns.len() == 0 {
        let input = match cli_clipboard::get_contents() {
            Ok(content) => content,
            Err(_) => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input.pop();
                input
            }
        };

        unpack(&input);
    } else {
        let packed = pack(args.patterns);

        let mut ctx = ClipboardContext::new().unwrap();
        match ctx.set_contents(packed.to_owned()) {
            Ok(_) => println!("\n🥐 content copied to clipboard 🥐"),
            Err(_) => println!("\n{}", packed),
        }
    }
}
