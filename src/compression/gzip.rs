//! Gzip encoding and decoding.
//! Unneccessary for image manipulation, however exists as a
//! test for the DEFLATE algorithm. For this reason, the
//! documentation is less exhaustive than in the rest of this
//! project.
use std::{fs, io, path::Path};

//      +------+
//      | GZIP |
//      +------+

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
