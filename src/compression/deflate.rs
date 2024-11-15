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
///         LZSS algorithm, a derivative of the LZ77 data compression algorithm.
///         Then, the data is compressed further using a fixed set of prefix codes.
///
///             BFINAL  BTYPE   BITSTREAM... EOB
///             1 bit   2 bits               9 bits
///
///     10 - Block Type 2: LZSS with Dynamic Codes
///             This block type once more uses the LZSS algorithm to compress data,
///         then uses dynamically created codes.
///
///             BFINAL  BTYPE   BITSTREAM... EOB
///             1 bit   2 bits               9 bits
///
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
