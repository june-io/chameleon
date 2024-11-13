use chameleon::compression;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = compression::GzipFile::build("./data/zlib/sunbears.gz")?;

    println!(
        "{:?}",
        compression::parse_block_header(file.deflate_blocks[0])
    );

    Ok(())
}
