use chameleon::compression;
use chameleon::compression::huffman::Coder;
use chameleon::compression::lzss::{self, LempelZiv};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read("./data/sunbears.txt").unwrap();

    let sunbear_huffman = compression::huffman::Huffman::encode(&file).unwrap();

    let sunbear_decoded = compression::huffman::Huffman::decode(&sunbear_huffman).unwrap();

    println!(
        "Before: {} bytes\n After: {} bytes\n Decoded: {}",
        file.len(),
        sunbear_huffman.len(),
        sunbear_decoded.len()
    );

    println!("{:?}", (file == sunbear_decoded));
    Ok(())
}
