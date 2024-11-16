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
