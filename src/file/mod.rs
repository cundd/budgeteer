mod file_writer;
mod file_reader;

pub use self::file_reader::FileReader;
pub use self::file_reader::LineParts;
pub use self::file_writer::FileWriter;
use std::{fs, env};
use std::path::{PathBuf, Path};
use error::{Res, Error};

pub fn normalize_file_path<P: AsRef<Path>>(path_input: P) -> Res<PathBuf> {
    let path = path_input.as_ref();
    if path.is_dir() {
        Err(Error::file_io("File path must not be a directory"))
    } else if path.exists() {
        Ok(fs::canonicalize(path)?)
    } else {
        match path.parent() {
            Some(parent) => {
                if !parent.is_dir() {
                    return prepend_current_working_directory(&path);
                }

                match fs::canonicalize(parent) {
                    Ok(_) => Ok(path.to_path_buf()),
                    Err(_) => Err(Error::file_io(format!(
                        "File parent directory {} not found",
                        parent.to_path_buf().to_string_lossy()
                    )))
                }
            }
            None => Err(Error::file_io("File path not found"))
        }
    }
}

fn prepend_current_working_directory(path: &&Path) -> Res<PathBuf> {
    match env::current_dir() {
        Ok(cwd) => {
            normalize_file_path(format!(
                "{}/{}",
                cwd.to_string_lossy(),
                path.to_string_lossy()
            ))
        }
        Err(_) => Err(Error::file_io("File has no parent directory"))
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert!(normalize_file_path(file!()).is_ok());
        assert_eq!(
            normalize_file_path(file!()).unwrap().to_string_lossy(),
            env!("CARGO_MANIFEST_DIR").to_owned() + "/" + file!()
        );
    }

    #[test]
    fn test_normalize_path_new() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Could not get current time")
            .subsec_nanos();
        let path = format!("{}/new-file-that-did-not-exist-before-{}.md", env!("CARGO_MANIFEST_DIR").to_owned(), suffix);
        assert!(normalize_file_path(&path).is_ok());
        assert_eq!(
            normalize_file_path(&path).unwrap().to_string_lossy(),
            path
        );
    }

    #[test]
    fn test_normalize_path_directory() {
        assert!(normalize_file_path(env!("CARGO_MANIFEST_DIR")).is_err());
    }
}
