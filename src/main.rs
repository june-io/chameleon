use chameleon::compression::huffman::{Coder, Huffman};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read("./data/sunbears.txt").unwrap();

    let sunbear_huffman = Huffman::encode(&file).unwrap();

    let sunbear_decoded = Huffman::decode(&sunbear_huffman).unwrap();

    println!(
        "Before: {} bytes\n After: {} bytes\n Decoded: {} bytes",
        file.len(),
        sunbear_huffman.len(),
        sunbear_decoded.len()
    );

    println!("Before == Decoded: {:?}", (file == sunbear_decoded));
    Ok(())
}
