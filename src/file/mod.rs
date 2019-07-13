mod file_writer;
mod file_reader;

pub use self::file_reader::FileReader;
pub use self::file_reader::LineParts;
pub use self::file_writer::FileWriter;
use std::fs;
use std::path::PathBuf;
use error::Res;

pub fn normalize_path(path: &str) -> Res<PathBuf> {
    Ok(fs::canonicalize(path)?)
}
