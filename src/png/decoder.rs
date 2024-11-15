use std::{
    error::Error,
    fmt::{self, Display},
    fs, io,
    path::{Path, PathBuf},
    str,
};

//      +--------+
//      | CONSTS |
//      +--------+

pub const PNG_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

//      +-----------+
//      | FILETYPES |
//      +-----------+

/// A struct containing a PNG image.
///
/// # Attributes
///
/// * 'data' - The Vec<u8> storing the raw data.
pub struct Png {
    pub data: PngData,
}

// Defines behavior related to creating Png structs
impl Png {
    /// Creates a Png struct from the given path.
    ///
    /// # Arguments
    ///
    /// * 'path' - The file path to the PNG file, can be any type that implements into path.
    ///
    /// # Returns
    ///
    /// A result containing either the constructed Png or a DecoderError.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Png, DecoderError> {
        let path = path.as_ref();

        let file_bytes = fs::read(path)?;

        if !is_png(file_bytes.clone()) {
            return Err(DecoderError::TypeError(format!("{:?} is not a PNG.", path)));
        }

        let data = PngData::build(file_bytes)?;

        Ok(Png { data })
    }
}

/// A struct containing the roughly parsed data of a PNG file.
///
/// # Arguments
///
/// * 'raw_data' - A Vec<u8> containing the raw byte data.
/// * 'ihdr' - An array storing the 13 byte IHDR chunk.
/// * 'plte' - Contains the optional PLTE chunk.
/// * 'IDAT' - Contains a vector of Vec<u8>'s containing the IDAT chunk/chunks.
pub struct PngData {
    pub raw_data: Vec<u8>,
    pub ihdr: Vec<u8>,
    pub plte: Option<Vec<u8>>,
    pub idat: Vec<u8>,
    pub crc: Vec<u8>,
    pub index: usize,
}

impl PngData {
    /// Takes in the raw PNG byte vector and splits it into the IHDR,
    /// PLTE, IDAT, and CRC chunks. As well as initializing the index
    /// for walking the data at the end of the PNG header.
    ///
    /// # Arguments
    ///
    /// * 'raw_data' - A Vec<u8> containing the raw byte data of the PNG file.
    ///
    /// # Returns
    ///
    /// A result containing either the built PngData struct or a DecoderError.
    pub fn build(raw_data: Vec<u8>) -> Result<Self, DecoderError> {
        let mut data = PngData {
            raw_data: raw_data.clone(),
            ihdr: Vec::with_capacity(13),
            plte: None,
            idat: Vec::new(),
            crc: Vec::new(),
            index: PNG_HEADER.len(),
        };

        let chunks = data.get_chunk_indexes().unwrap();

        data.ihdr = raw_data[chunks[0].0..chunks[0].1].to_vec();
        if chunks[1] != (0, 0) {
            data.plte = Some(raw_data[chunks[1].0..chunks[1].1].to_vec());
        }
        data.idat = raw_data[chunks[2].0..chunks[2].1].to_vec();
        data.crc = raw_data[chunks[3].0..chunks[3].1].to_vec();

        Ok(data)
    }
    pub fn walk(&mut self, length: usize) -> Result<Vec<u8>, DecoderError> {
        if self.index + length > self.raw_data.len() {
            return Err(DecoderError::NoMoreChunks(self.index + length));
        }

        let chunk = self.raw_data[self.index..self.index + length].to_vec();
        self.index += 1;
        Ok(chunk)
    }
    pub fn get_chunk_indexes(&mut self) -> Result<[(usize, usize); 4], DecoderError> {
        let mut ihdr_index: (usize, usize) = (0, 0);
        let mut plte_index: (usize, usize) = (0, 0);
        let mut idat_index: (usize, usize) = (0, 0);
        let mut crc_index: (usize, usize) = (0, self.raw_data.len());
        while let Ok(v) = self.walk(4) {
            let str_result = str::from_utf8(v.as_slice());
            match str_result {
                Ok("IHDR") => {
                    ihdr_index.0 = self.index + 4;
                    ihdr_index.1 = self.index + 4 + 13;
                }
                Ok("PLTE") => {
                    plte_index.0 = self.index + 4;
                }
                Ok("IDAT") => {
                    println!("FOUND IDAT");
                    if idat_index == (0, 0) {
                        if plte_index != (0, 0) {
                            idat_index.0 = self.index + 4;
                            plte_index.1 = self.index - 1;
                        } else {
                            idat_index.0 = self.index + 4;
                        }
                    }
                }
                Ok("IEND") => {
                    idat_index.1 = self.index - 1;
                    crc_index.0 = self.index + 4;
                }
                _ => {}
            }
        }

        Ok([ihdr_index, plte_index, idat_index, crc_index])
    }
}

//       +--------+
//       | ERRORS |
//       +--------+

/// Enum containing possible errors raised by the decoding process.
///
/// # Fields
///
/// * 'TypeError' - Used when the decoder is called on a file with a different type,
///             or an invalid file of the correct type. Takes a String as an argument
///             to store the name of the file causing the error.
/// * 'IoError' - Wrapper for io::Error for errors while reading and writing to files.
///
/// # Examples
///
/// '''
/// fn get_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, DecoderError> {
///     let png_header: [u8; 8] = [137, 80, 78, 71, 13, 10, 27, 10];
///     let path = path.as_ref();
///     
///     // Either works or returns an DecoderError::IoError containing
///     // the error thrown by fs::read.
///     let file_bytes = match fs::read(path)?;
///     
///     // Checks if file_bytes starts with the PNG header and retuns
///     // a TypeError if not.
///     if !bytes[0..8] == png_header {
///         return Err(DecoderError::TypeError(format!("{:?}", path)));
///     }
///
///     Ok(file_bytes)
/// }
/// '''
#[derive(Debug)]
pub enum DecoderError {
    TypeError(String),
    IoError(io::Error),
    NoMoreChunks(usize),
}

// Defines how DecoderErrors are displayed.
impl Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecoderError::TypeError(e) => {
                write!(
                    f,
                    "Error: Attempted to decode incompatible type as png, '{e}'."
                )
            }
            DecoderError::IoError(e) => {
                write!(f, "Error: decoder cause an io::Error, '{e}'")
            }
            DecoderError::NoMoreChunks(v) => {
                write!(f, "Error: No more chunks left to iterate over, reached end of file at index '{v}'")
            }
        }
    }
}

// Allows for conversion from io::Error to DecoderError.
impl From<io::Error> for DecoderError {
    fn from(error: io::Error) -> Self {
        DecoderError::IoError(error)
    }
}

// Implements the Error interface for CliError.
impl Error for DecoderError {}

//      +----------+
//      | UTILITES |
//      +----------+

/// Takes in a byte array and checks for the png file header.
///
/// # Arguments
///
/// * 'bytes' - Vec<u8> containing the bytes of the input file.
///
/// # Returns
///
/// A boolean value, true if the input is a png, and false if
/// either the given byte array is too small, or the png file
/// header is not found. If somehow neither cases are true,
/// returns false.
pub fn is_png(bytes: Vec<u8>) -> bool {
    if bytes.len() < PNG_HEADER.len() {
        return false;
    }

    if bytes[0..8] == PNG_HEADER {
        return true;
    }
    false
}
