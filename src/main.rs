use chameleon::compression::{self, LempelZiv};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = compression::GzipFile::build("./data/gzip/sunbears.gz")?;

    let uncompressed_file = fs::read("./data/gzip/sunbears")?;

    let mut lzss = LempelZiv::new(uncompressed_file);

    lzss.compress();

    Ok(())
}
