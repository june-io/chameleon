use std::fs;
use std::io;
use std::path::Path;

pub struct GzipFile {
    pub header: Vec<u8>,
    pub deflate_blocks: Vec<u8>,
    pub footer: Vec<u8>,
}

impl GzipFile {
    pub fn build<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file_bytes = fs::read(path.as_ref())?;

        // name:    MAGIC1  MAGIC2  CM      FLAGS   MTIME   XFL     OS
        // bytes:   1       1       1       1       4       1       1
        let header = file_bytes[0..10].to_vec();
        let deflate_blocks = file_bytes[10..file_bytes.len() - 8].to_vec();
        let footer = file_bytes[file_bytes.len() - 8..file_bytes.len()].to_vec();

        Ok(GzipFile {
            header,
            deflate_blocks,
            footer,
        })
    }
}

/// Accepts the first byte of a DEFLATE block and extracts
/// the BFINAL and BTYPE values.
///
/// The first three bits of a DEFLATE block contain if the
/// block is the last and what type it is. The types are
/// as follows:
///
///     00 - Block Type 0: Store
///             This block type stores uncompressed data. The header contains not
///         only the regular three bits but also the length, and somewhat bizarrely
///         the bitwise complement of the length. There is also 5 bits of padding
///         added after the first three bits to byte-align the bitstream.
///
///             BFINAL  BTYPE   PAD     LEN    ~LEN    BITSTREAM...
///             1 bit   2 bits  5 bits  16 bits 16 bits
///     
///     01 - Block Type 1: LZSS with Fixed Codes
///             This block type stores data which has been compressed using the
///         LZSS algorithm, which is a derivative of the LZ77 data compression
///         algorithm.
/// # Arguments
///
/// * 'byte' - An u8 representing the first byte of the block.
///
/// # Returns
///
/// A tuple containing BFINAL as a bool in the first field,
/// and BTYPE as an u8 in the second field.
pub fn parse_block_header(byte: u8) -> (bool, u8) {
    // Grab the first bit.
    let bfinal = (byte & 0b0000_0001) != 0;
    // Grab the second and third bits.
    let btype = (byte & 0b0000_0110) >> 1;

    (bfinal, btype)
}

pub fn decompress<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let path = path.as_ref();
    let file_bytes = fs::read(path)?;

    println!("{:?}", file_bytes);

    Ok(file_bytes)
}
