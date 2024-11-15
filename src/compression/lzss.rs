//! An implementation of the LZSS algorithm which works with the
//! DEFLATE algorithm

/// Representation of the length codes given in section 3.2.5 of RFC 1951.
/// In the format:
///
///     [SYMBOL, OFFSET BITS, RANGE START, RANGE END].
///
/// So, to encode the string "ABBBBB" first the EOB marker 256 will be added
/// to the end of the data, then the two literals A and B will be pushed into
/// the bitstream, then the length code symbol, in this case 258 to represent
/// the 4 B's we want the distance code 0 to represent the backwards distance 1.
///
///     A B B B B B -> A B B B B B 256 -> A B 4:1 256 -> 65 66 258 0 256
///
/// The final symbols are then processed through either the fixed prefix codes
/// if the block is of type 1, or the dynamic prefix codes given, if the block
/// is of type 2.
pub const LENGTH_CODE_RANGES: [[u16; 4]; 29] = [
    [257, 0, 3, 3],
    [258, 0, 4, 4],
    [259, 0, 5, 5],
    [260, 0, 6, 6],
    [261, 0, 7, 7],
    [262, 0, 8, 8],
    [263, 0, 9, 9],
    [264, 0, 10, 10],
    [265, 1, 11, 12],
    [266, 1, 13, 14],
    [267, 1, 15, 16],
    [268, 1, 17, 18],
    [269, 2, 19, 22],
    [270, 2, 23, 26],
    [271, 2, 27, 30],
    [272, 2, 31, 34],
    [273, 3, 35, 42],
    [274, 3, 43, 50],
    [275, 3, 51, 58],
    [276, 3, 59, 66],
    [277, 4, 67, 82],
    [278, 4, 83, 98],
    [279, 4, 99, 114],
    [280, 4, 115, 130],
    [281, 5, 131, 162],
    [282, 5, 163, 194],
    [283, 5, 195, 226],
    [284, 5, 227, 257],
    [285, 0, 258, 258],
];

/// Representation of the distance codes given in section 3.2.5 of RFC 1951.
/// In the format:
///
///     [SYMBOL, EXTRA BITS, DISTANCE RANGE START, DISTANCE RANGE END]
///
/// The extra bits are used to define which element of the range is used.
/// So, for distance code 4 there is 1 extra bit which is used to specify
/// rather the desired distance is 5 or 6.
pub const DISTANCE_CODE_RANGES: [[u16; 4]; 30] = [
    [0, 0, 1, 1],
    [1, 0, 2, 2],
    [2, 0, 3, 3],
    [3, 0, 4, 4],
    [4, 1, 5, 6],
    [5, 1, 7, 8],
    [6, 2, 9, 12],
    [7, 2, 13, 16],
    [8, 3, 17, 24],
    [9, 3, 25, 32],
    [10, 4, 33, 48],
    [11, 4, 49, 64],
    [12, 5, 65, 96],
    [13, 5, 97, 128],
    [14, 6, 129, 192],
    [15, 6, 193, 256],
    [16, 7, 257, 384],
    [17, 7, 385, 512],
    [18, 8, 513, 768],
    [19, 8, 769, 1024],
    [20, 9, 1025, 1536],
    [21, 9, 1537, 2048],
    [22, 10, 2049, 3072],
    [23, 10, 3073, 4096],
    [24, 11, 4097, 6144],
    [25, 11, 6145, 8192],
    [26, 12, 8193, 12288],
    [27, 12, 12289, 16384],
    [28, 13, 16385, 24576],
    [29, 13, 24577, 32768],
];

pub struct LempelZiv {
    pub data: Vec<u8>,
    // Default for DEFLATE is 32768
    pub buffer_size: usize,
}

impl LempelZiv {
    /// Takes in the data to compress/decompress and creates a LempelZiv struct
    /// with the default buf_size defined by RFC 1951 of 32768 chars/bytes.
    ///
    /// # Arguments
    ///
    /// * 'data' - A Vec<u8> containing the data to compress/decompress.
    ///
    /// # Returns
    ///
    /// A LempelZiv struct with the given data with a search buffer size of 32768.
    pub fn new(data: Vec<u8>) -> Self {
        LempelZiv {
            data,
            buffer_size: 32768,
        }
    }
    /// Similar to LempelZiv::new(), however allows for a custom search buffer
    /// size.
    ///
    /// # Arguments
    ///
    /// * 'data' - A vec<u8> containing the data to compress/decompress.
    /// * 'buf_size' - A usize value representing the size in bytes of the search
    ///             buffer.
    ///
    /// # Returns
    ///
    /// A LempelZiv struct with the given data and search buffer size.
    pub fn with_buf_size(data: Vec<u8>, buffer_size: usize) -> Self {
        LempelZiv { data, buffer_size }
    }
    pub fn compress(&mut self) -> Vec<u8> {
        // Creates the search_buffer and check_for vectors with the capacity
        // of the input data length, because the search buffer size cannot
        // surpass the length of the input data, and the checking vector cannot
        // surpass the length fo the search_buffer.
        let mut search_buffer: Vec<u8> = Vec::with_capacity(self.data.len());
        let mut check_for: Vec<u8> = Vec::with_capacity(self.data.len());

        // Clones self.data into an owned enumerated iterator.
        let data = self.data.clone().into_iter().enumerate();

        // Iterate over data while storing both the index i, and the value byte.
        for (i, byte) in data {
            check_for.push(byte);
            let mut index = self.in_array(&check_for, &search_buffer);

            if index == -1 || i == self.data.len() - 1 {
                if check_for.len() > 1 {
                    index = self.in_array(&check_for[0..check_for.len() - 1], &search_buffer);
                    let offset = i as isize - index - check_for.len() as isize;
                    let length = check_for.len();

                    let token = format!("{}, {}", offset, length);
                    if token.len() > length {
                        println!("{}, {}", i, byte as char);
                    } else {
                        println!("{}", token);
                    }
                } else {
                    println!("{}, {}", i, byte as char);
                }
                check_for = Vec::with_capacity(self.data.len());
            }

            search_buffer.push(byte);
        }

        vec![0]
    }
    /// Accepts a byte vector containing a needle to search for, and a byte
    /// vector as a haystack. Then iterates through the haystack and checks
    /// if offset is more than the length of the needle, if not checks if the
    /// byte at the offset position in the needle equals the current haystack
    /// byte and if so, increment offset, if not set offset to 0.
    ///
    /// # Arguments
    ///
    /// * 'check_for' - A Vec<u8> containing the pattern to search for within
    ///             check_from.
    /// * 'check_from' - The Vec<u8> to search for check_for within.
    ///
    /// # Returns
    ///
    /// An isize value representing the index within check_from that check_for
    /// appears, if it does not appear, returns -1.
    fn in_array(&self, check_for: &[u8], check_from: &[u8]) -> isize {
        let check_from_iter = check_from.iter().enumerate();
        let mut offset = 0;

        for (index, byte) in check_from_iter {
            if check_for.len() <= offset {
                return index as isize - check_for.len() as isize;
            }

            if &check_for[offset] == byte {
                offset += 1;
            } else {
                offset = 0;
            }
        }
        -1
    }
}
