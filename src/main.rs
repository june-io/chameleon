use chameleon::bits::bitstream::BitStream;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut bitstre = BitStream::new();

    bitstre.len = 11;
    bitstre.bytes = vec![255u8, 0b1110_0000];

    println!("{}", bitstre);

    for _ in 0..10 {
        bitstre.push(1);
    }
    println!("{}", bitstre);
    Ok(())
}
