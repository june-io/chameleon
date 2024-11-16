//! Implementation of regular bitstreams as well as conversion
//! from a bitstream to bytes as utilized in RFC 1951.
//!

use std::fmt::Display;

/// Struct representing a bitstream/array of bits. Bits are
/// stored in a byte array, stored with a length to state how
/// many bits of that byte array are part of the bitstream. By
/// default bitstreams are big-endian but contain a function to
/// flip them to be little endian.
///
/// # Fields
///
/// * 'len' - A u32 value representing the number of bits in
///         the bistream.
/// * 'bits' - A Vec<u8> containing bytes within which the bitstream
///         is stored, the vector will always be of length len/8 rounded
///         up. Bits are stored left to right, with the most significant
///         bit at the left.
///
/// # Methods
///
/// * 'new' - Generates a new empty bitstream with the specified endianness.
/// * 'push' - Takes in a 0 or 1 and pushes it to the right side of the bitstream.
///
/// # Examples
///
/// '''
/// let stream = BitStream::new();
///
/// print!("{}", bitstream); // output: 0: 00000000
/// stream.push(1);
/// print!("{}", bitstream); // output: 0: 10000000
///
/// '''
pub struct BitStream {
    pub len: u32,
    pub bytes: Vec<u8>,
}

impl BitStream {
    /// Creates a new empty bitstream.
    ///
    /// # Returns
    ///
    /// The generated empty bitstream of the given endianness.
    ///
    /// # Examples
    ///
    /// '''
    /// let stream = BitStream::new();
    /// stream.push(0);
    /// '''
    pub fn new() -> Self {
        Self {
            len: 0,
            bytes: Vec::new(),
        }
    }
    /// Takes in a 0 or 1 and pushes it to the least significant
    /// end of the bistream, so for little endian bitstreams, the
    /// first bit of the first byte in the array will be altered
    ///
    /// #
    pub fn push(&mut self, bit: u8) {
        // Check if bit is a 0 or 1 and if not treat it as a 1 while
        // printing a warning.
        let normalized_bit: u8 = match bit {
            0 => 0,
            1 => 1,
            _ => {
                eprintln!(
                    "Warning: Non-binary value given to .push(), value has been corrected to a 1."
                );
                1
            }
        };

        // Initialize an empty byte to populate with the new values.
        let bit_index: u8 = (self.len % 8) as u8;
        println!("bitindex: {}", bit_index);

        // If the last byte in the byte array is not filled.
        if bit_index != 0 {
            // Take the last byte and shift out the unfilled bits
            // if bit_index = 3 and byte = 0xE0
            // 0b1110_0000 << 8 - bit_index (5)
            // 0b0000_0111 >>= 1;
            // bit-index += 1;
            // 0b0000_1111 >> 8 - bit_index (4)
            // final = 0b1111_0000
            let shift: u8 = 8 - bit_index - 1;
            let len = self.bytes.len() - 1;
            self.bytes[len] =
                ((self.bytes[self.bytes.len() - 1] >> shift) | normalized_bit) << shift;
            self.len += 1;
        } else {
            // If the last byte is full, define the working byte as
            // normalized_bit in shifted over 7 to make it the most
            // significant bit, then push it to the byte array before
            // incrementing len.
            let working_byte = normalized_bit << 7;
            self.bytes.push(working_byte);
            self.len += 1;
        }
    }
    /// RFC 1951 Section 3.1.1 describes the process of packing
    /// the bits into bytes as follows:
    ///
    ///     * Data elements are packed into bytes in order of
    ///     increasing bit number within the byte, i.e., starting
    ///     with the least-significant bit of the byte.
    ///
    ///     * Data elements other than Huffman codes are packed
    ///     starting with the least-significant bit of the data
    ///     element.
    ///
    ///     * Huffman codes are packed starting with the most-
    ///    significant bit of the code.
    ///
    /// So, to reverse the parsing process, from the final compressed
    /// bytes to the original bit stream the process would be:
    ///
    ///     * Print out the compressed data starting with the first byte
    ///     at the right and proceeding left.
    ///
    ///     [0xED, 0x02] -> [0x02, 0xED]
    ///
    ///     * Convert to bits with the most significant bit at the left.
    ///
    ///     [0x02, 0xED] -> [0b0000_0010, 0b1110_1101]
    ///
    ///     * Now the bitstream can be read from right to left, so visualizing
    ///     it from left to right looks like:
    ///
    ///      -> 1011_0111_01
    ///
    /// This function performs the inverse of this operation assuming.
    /// the huffman codes have already been pushed in with the right orientation.
    /// Anticipates byte aligned data so unfilled bytes are truncated.
    pub fn to_rfc_bytes(&self) -> Vec<u8> {
        todo!();
    }
}

impl Default for BitStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for BitStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, byte) in self.bytes.iter().enumerate() {
            writeln!(f, "{}: {:08b}", i, byte)?;
        }
        Ok(())
    }
}
